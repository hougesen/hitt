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
fn parse_method_input(chars: &mut core::iter::Enumerate<std::str::Chars>) -> String {
    let mut method = String::new();

    for (_i, c) in chars {
        if c.is_whitespace() {
            if !method.is_empty() {
                return method;
            }
        } else {
            method.push(c);
        }
    }

    method
}

#[inline]
fn parse_uri_input(chars: &mut core::iter::Enumerate<std::str::Chars>) -> String {
    let mut url = String::new();

    for (_i, c) in chars {
        if c.is_whitespace() {
            if !url.is_empty() {
                return url;
            }
        } else {
            url.push(c);
        }
    }

    url
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

fn parse_tokens(buffer: String) -> Result<Vec<RequestToken>, RequestParseError> {
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
                    let method = http::method::Method::from_str(&parse_method_input(&mut chrs))?;

                    tokens.push(RequestToken::Method(method));

                    let uri = (parse_uri_input(&mut chrs)).parse::<http::uri::Uri>()?;

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
pub fn parse_requests(buffer: String) -> Result<Vec<HittRequest>, RequestParseError> {
    let mut requests = Vec::new();

    let tokens = parse_tokens(buffer)?;

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

    const HTTP_METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

    #[test]
    fn it_should_parse_http_method_correctly() {
        let url = "https://mhouge.dk";

        HTTP_METHODS.iter().for_each(|method| {
            let expected_method = http::Method::from_str(method).expect("m is a valid method");

            let parsed_requests =
                parse_requests(format!("{method} {url}")).expect("request should be valid");

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
}
