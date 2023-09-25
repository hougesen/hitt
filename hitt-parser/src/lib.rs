use std::str::FromStr;

#[derive(Debug)]
pub enum RequestParseError {
    InvalidHttpMethod(http::method::InvalidMethod),
    InvalidUri(http::uri::InvalidUri),
    MissingMethod,
    MissingUri,
}

impl From<http::method::InvalidMethod> for RequestParseError {
    fn from(value: http::method::InvalidMethod) -> Self {
        Self::InvalidHttpMethod(value)
    }
}

impl From<http::uri::InvalidUri> for RequestParseError {
    fn from(value: http::uri::InvalidUri) -> Self {
        RequestParseError::InvalidUri(value)
    }
}

#[inline]
fn parse_method_input(
    chars: &mut core::iter::Enumerate<std::str::Chars>,
) -> Result<http::method::Method, http::method::InvalidMethod> {
    let mut method = String::new();

    for (_i, c) in chars {
        if c.is_whitespace() {
            if !method.is_empty() {
                break;
            }
        } else {
            method.push(c);
        }
    }

    http::method::Method::from_str(&method.to_uppercase())
}

#[cfg(test)]
mod test_parse_method_input {
    use crate::parse_method_input;

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    #[test]
    fn it_should_accept_valid_methods() {
        for method_input in HTTP_METHODS {
            let input = format!("{method_input} https://mhouge.dk HTTP/2");

            let parsed_method = parse_method_input(&mut input.chars().enumerate())
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }

    #[test]
    fn it_should_ignore_case() {
        for method_input in HTTP_METHODS {
            let input = format!("{} https://mhouge.dk HTTP/2", method_input.to_lowercase());

            let parsed_method = parse_method_input(&mut input.chars().enumerate())
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }
}

#[inline]
fn parse_uri_input(
    chars: &mut core::iter::Enumerate<std::str::Chars>,
) -> Result<http::uri::Uri, http::uri::InvalidUri> {
    let mut uri = String::new();

    for (_i, c) in chars {
        if c.is_whitespace() {
            if !uri.is_empty() {
                break;
            }
        } else {
            uri.push(c);
        }
    }

    http::uri::Uri::from_str(&uri)
}

#[cfg(test)]
mod test_parse_uri_input {
    use crate::parse_uri_input;

    #[test]
    fn it_should_be_able_to_parse_uris() {
        let input_uris = [
            "https://mhouge.dk/",
            "https://goout.dk/",
            "https://mhouge.dk?key=value",
        ];

        for input_uri in input_uris {
            let result = parse_uri_input(&mut format!("{input_uri} HTTP/2").chars().enumerate());

            assert!(result.is_ok());
        }
    }

    #[test]
    fn it_should_ignore_leading_spaces() {
        let input_uri = "https://mhouge.dk/";

        let result =
            parse_uri_input(&mut format!("         {input_uri} HTTP/2.0").chars().enumerate())
                .expect("it should return a valid uri");

        assert_eq!(result.to_string(), input_uri)
    }

    #[test]
    fn it_should_reject_invalid_uris() {
        let invalid_uris = ["m:a:d:s"];

        for invalid_uri in invalid_uris {
            parse_uri_input(&mut format!("{invalid_uri} HTTP/2").chars().enumerate())
                .expect_err("it should return an error");
        }
    }

    #[test]
    fn it_should_support_query_paramers() {
        let input_uri = "https://mhouge.dk/";

        for i in 0..10 {
            let result =
                parse_uri_input(&mut format!("{input_uri}?key{i}=value{i}").chars().enumerate())
                    .expect("it should return a valid uri");

            assert_eq!(result.to_string(), format!("{input_uri}?key{i}=value{i}"));
        }
    }
}

enum ParserMode {
    Request,
    Headers,
    Body,
}

#[derive(Debug)]
struct HeaderToken {
    key: http::HeaderName,
    value: http::HeaderValue,
}

#[inline]
fn parse_header(line: core::iter::Enumerate<std::str::Chars>) -> Option<HeaderToken> {
    let mut key = String::new();
    let mut value = String::new();
    let mut is_key = true;

    for (_index, c) in line {
        match c {
            ':' => {
                if is_key {
                    is_key = false;
                } else {
                    value.push(c);
                }
            }
            _ => {
                if is_key {
                    key.push(c);
                } else if !(value.is_empty() && c == ' ') {
                    value.push(c)
                }
            }
        }
    }

    if !key.is_empty() {
        let k = http::HeaderName::from_str(&key).expect("Unable to parse key as header key");
        let v = http::HeaderValue::from_str(&value).expect("Unable to parse value as header value");

        return Some(HeaderToken { key: k, value: v });
    }

    None
}

#[cfg(test)]
mod test_parse_header {
    use std::str::FromStr;

    use crate::parse_header;

    #[test]
    fn it_should_return_valid_headers() {
        for i in 0..10 {
            let line = format!("header{i}: value{i}");

            let result = parse_header(line.chars().enumerate())
                .expect("It should be able to parse valid headers");

            let expected_key = http::HeaderName::from_str(&format!("header{i}"))
                .expect("expected key to be valid");

            assert_eq!(result.key, expected_key);

            let expected_value = http::HeaderValue::from_str(&format!("value{i}"))
                .expect("expected value to be valid");

            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    fn it_should_ignore_empty_lines() {
        let result = parse_header("".chars().enumerate());

        assert!(result.is_none());
    }
}

#[derive(Debug)]
enum RequestToken {
    Method(http::method::Method),
    Uri(http::uri::Uri),
    Header(HeaderToken),
    Body(Option<String>),
}

impl From<http::method::Method> for RequestToken {
    fn from(value: http::method::Method) -> Self {
        RequestToken::Method(value)
    }
}

impl From<http::uri::Uri> for RequestToken {
    fn from(value: http::uri::Uri) -> Self {
        RequestToken::Uri(value)
    }
}

impl From<HeaderToken> for RequestToken {
    fn from(value: HeaderToken) -> Self {
        RequestToken::Header(value)
    }
}

fn tokenize(buffer: &str) -> Result<Vec<RequestToken>, RequestParseError> {
    let mut tokens: Vec<RequestToken> = Vec::new();

    let mut parser_mode = ParserMode::Request;

    let mut body_parts: Vec<&str> = Vec::new();

    for (_index, line) in buffer.lines().enumerate() {
        let trimmed_line = line.trim();

        // check if line is comment (#) OR requests break (###)
        if trimmed_line.starts_with('#') {
            if trimmed_line.starts_with("###") {
                tokens.push(if body_parts.is_empty() {
                    RequestToken::Body(None)
                } else {
                    RequestToken::Body(Some(String::new()))
                });

                body_parts.clear();
                parser_mode = ParserMode::Request;
            }

            continue;
        }

        // check if line is comment (//)
        if trimmed_line.starts_with("//") {
            continue;
        }

        match &parser_mode {
            ParserMode::Request => {
                if !trimmed_line.is_empty() {
                    let mut chrs = line.chars().enumerate();
                    let method = parse_method_input(&mut chrs)?;

                    tokens.push(RequestToken::Method(method));

                    let uri = parse_uri_input(&mut chrs)?;

                    tokens.push(RequestToken::Uri(uri));

                    parser_mode = ParserMode::Headers;
                }
            }

            ParserMode::Headers => {
                if line.trim().is_empty() {
                    parser_mode = ParserMode::Body;
                } else if let Some(header_token) = parse_header(line.chars().enumerate()) {
                    tokens.push(RequestToken::Header(header_token));
                }
            }

            ParserMode::Body => {
                body_parts.push(line);
            }
        };
    }

    if !body_parts.is_empty() {
        tokens.push(RequestToken::Body(Some(body_parts.join(""))));
    }

    Ok(tokens)
}

#[cfg(test)]
mod test_tokenize {

    use crate::{tokenize, RequestToken};

    #[test]
    fn should_return_a_list_of_tokens() {
        let method_input = "GET";
        let uri_input = "https://mhouge.dk/";

        let header1_key = "content-type";
        let header1_value = "application/json";
        let body_input = "{ \"key\": \"value\"  }";

        let input_request =
            format!("{method_input} {uri_input}\n{header1_key}: {header1_value}\n\n{body_input}");

        let tokens = tokenize(&input_request).expect("it to return Result<Vec<RequestToken>>");

        assert_eq!(tokens.len(), 4);

        for token in tokens {
            match token {
                RequestToken::Uri(uri_token) => assert_eq!(uri_input, uri_token.to_string(),),
                RequestToken::Method(method_token) => {
                    assert_eq!(method_input, method_token.as_str())
                }
                RequestToken::Header(header_token) => {
                    assert_eq!(header1_key, header_token.key.to_string());

                    assert_eq!(header1_value, header_token.value.to_str().unwrap());
                }

                RequestToken::Body(body) => {
                    assert!(body.is_some());

                    let body_inner = body.expect("body to be defined");

                    assert_eq!(body_input, body_inner);
                }
            }
        }
    }

    #[test]
    fn it_should_be_able_to_parse_multiple_requests() {
        let input = r"
GET https://mhouge.dk/ HTTP/1.1
x-test-header: test value

###

GET https://mhouge.dk/ HTTP/2
x-test-header: test value

###

GET https://mhouge.dk/ HTTP/3
x-test-header: test value
###
";
        let tokens = tokenize(input).expect("it to return a list of tokens");

        assert_eq!(12, tokens.len());

        for token in tokens {
            match token {
                RequestToken::Method(method_token) => assert_eq!("GET", method_token.as_str()),

                RequestToken::Uri(uri_token) => {
                    assert_eq!("https://mhouge.dk/", uri_token.to_string())
                }

                RequestToken::Header(header_token) => {
                    assert_eq!("x-test-header", header_token.key.as_str());

                    assert_eq!(
                        "test value",
                        header_token
                            .value
                            .to_str()
                            .expect("header field to be defined")
                    );
                }

                RequestToken::Body(body_token) => assert!(body_token.is_none()),
            }
        }
    }
}

#[derive(Debug)]
pub struct HittRequest {
    pub method: http::method::Method,
    pub uri: http::uri::Uri,
    pub headers: http::HeaderMap,
    pub body: Option<String>,
}

#[derive(Default)]
struct PartialHittRequest {
    method: Option<http::method::Method>,
    uri: Option<http::uri::Uri>,
    headers: http::HeaderMap,
    body: Option<String>,
}

impl PartialHittRequest {
    fn build(self) -> Result<HittRequest, RequestParseError> {
        match self.method {
            Some(method) => match self.uri {
                Some(uri) => Ok(HittRequest {
                    method,
                    uri,
                    headers: self.headers,
                    body: self.body,
                }),
                None => Err(RequestParseError::MissingUri),
            },
            None => Err(RequestParseError::MissingMethod),
        }
    }
}

#[inline]
pub fn parse_requests(buffer: &str) -> Result<Vec<HittRequest>, RequestParseError> {
    let mut requests = Vec::new();

    let tokens = tokenize(buffer)?;

    let mut p = PartialHittRequest::default();

    for token in tokens {
        match token {
            RequestToken::Method(method) => {
                if p.method.is_some() {
                    requests.push(p.build()?);

                    p = PartialHittRequest::default();
                }

                p.method = Some(method);
            }
            RequestToken::Uri(uri) => {
                p.uri = Some(uri);
            }
            RequestToken::Header(header) => {
                p.headers.insert(header.key, header.value);
            }
            RequestToken::Body(body) => {
                p.body = body;

                requests.push(p.build()?);

                p = PartialHittRequest::default();
            }
        };
    }

    if p.method.is_some() {
        requests.push(p.build()?);
    };

    Ok(requests)
}

#[cfg(test)]
mod test_parse_requests {
    use std::str::FromStr;

    use crate::parse_requests;

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    #[test]
    fn it_should_parse_http_method_correctly() {
        let url = "https://mhouge.dk";

        HTTP_METHODS.iter().for_each(|method| {
            let expected_method = http::Method::from_str(method).expect("m is a valid method");

            let parsed_requests =
                parse_requests(&format!("{method} {url}")).expect("request should be valid");

            println!("parsed_requests: {:#?}", parsed_requests);

            assert!(parsed_requests.len() == 1);

            let first_request = &parsed_requests[0];

            assert_eq!(expected_method, first_request.method);

            let expected_uri = url.parse::<http::uri::Uri>().expect("url should be valid");

            assert_eq!(expected_uri, first_request.uri);

            assert_eq!(0, first_request.headers.len());

            assert_eq!(None, first_request.body);
        });
    }

    #[test]
    fn it_should_be_able_to_parse_requests() {
        let method_input = "GET";
        let uri_input = "https://mhouge.dk/";

        let header1_key = "content-type";
        let header1_value = "application/json";
        let body_input = "{ \"key\": \"value\"  }";

        let input_request =
            format!("{method_input} {uri_input}\n{header1_key}: {header1_value}\n\n{body_input}");

        let result = parse_requests(&input_request).expect("it to return a list of requests");

        assert!(result.len() == 1);

        let request = &result[0];

        assert_eq!(method_input, request.method.as_str());

        assert_eq!(uri_input, request.uri.to_string());

        let body_inner = request.body.clone().expect("body to be defined");

        assert_eq!(body_inner, body_input);

        assert_eq!(1, request.headers.len());

        let header1_output = request
            .headers
            .get(header1_key)
            .expect("header1_key to exist");

        assert_eq!(header1_value, header1_output.to_str().unwrap(),);
    }

    #[test]
    fn it_should_be_able_to_parse_multiple_requests() {
        let input = r"
GET https://mhouge.dk/ HTTP/1.1

###

GET https://mhouge.dk/ HTTP/2

###

GET https://mhouge.dk/ HTTP/3

###
";

        let requests = parse_requests(input).expect("to get a list of requests");

        assert_eq!(3, requests.len());

        for request in requests {
            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());
        }
    }
}
