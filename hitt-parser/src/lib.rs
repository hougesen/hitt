use error::RequestParseError;
use header::{parse_header, HeaderToken};
use method::parse_method_input;
use uri::parse_uri_input;
use variables::parse_variable_declaration;
use version::parse_http_version;

pub mod error;
mod header;
mod method;
mod uri;
mod variables;
mod version;

enum ParserMode {
    Request,
    Headers,
    Body,
}

#[derive(Debug)]
enum RequestToken {
    Method(http::method::Method),
    Uri(http::uri::Uri),
    HttpVersion(http::version::Version),
    Header(HeaderToken),
    Body(Option<String>),
}

#[inline]
fn tokenize(buffer: &str) -> Result<Vec<RequestToken>, RequestParseError> {
    let mut tokens: Vec<RequestToken> = Vec::new();

    let mut parser_mode = ParserMode::Request;

    let mut body_parts: Vec<&str> = Vec::new();

    let mut vars = std::collections::HashMap::new();

    for line in buffer.lines() {
        let trimmed_line = line.trim();

        // check if line is comment (#) OR requests break (###)
        if trimmed_line.starts_with('#') {
            if trimmed_line.starts_with("###") {
                tokens.push(if body_parts.is_empty() {
                    RequestToken::Body(None)
                } else {
                    RequestToken::Body(Some(body_parts.join("\n")))
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

        if trimmed_line.starts_with('@') {
            let mut chrs = trimmed_line.chars().enumerate();

            // move forward once since we don't care about the '@'
            chrs.next();

            if let Some((name, value)) = parse_variable_declaration(&mut chrs) {
                vars.insert(name, value);
                continue;
            }
        }

        match parser_mode {
            ParserMode::Request => {
                if !trimmed_line.is_empty() {
                    let mut chrs = line.chars().enumerate();
                    let method = parse_method_input(&mut chrs)?;

                    tokens.push(RequestToken::Method(method));

                    let uri = parse_uri_input(&mut chrs, &vars)?;

                    tokens.push(RequestToken::Uri(uri));

                    if let Some(http_version) = parse_http_version(&mut chrs) {
                        tokens.push(RequestToken::HttpVersion(http_version));
                    }

                    parser_mode = ParserMode::Headers;
                }
            }

            ParserMode::Headers => {
                if line.trim().is_empty() {
                    parser_mode = ParserMode::Body;
                } else if let Some(header_token) = parse_header(line.chars().enumerate())? {
                    tokens.push(RequestToken::Header(header_token));
                }
            }

            ParserMode::Body => {
                body_parts.push(line);
            }
        };
    }

    if !body_parts.is_empty() {
        tokens.push(RequestToken::Body(Some(body_parts.join("\n"))));
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
        let http_version = "HTTP/2";
        let header1_key = "content-type";
        let header1_value = "application/json";
        let body_input = "{ \"key\": \"value\"  }";

        let input_request =
            format!("{method_input} {uri_input} {http_version}\n{header1_key}: {header1_value}\n\n{body_input}");

        let tokens = tokenize(&input_request).expect("it to return Result<Vec<RequestToken>>");

        assert_eq!(tokens.len(), 5);

        for token in tokens {
            match token {
                RequestToken::Uri(uri_token) => assert_eq!(uri_input, uri_token.to_string()),
                RequestToken::Method(method_token) => {
                    assert_eq!(method_input, method_token.as_str());
                }
                RequestToken::Header(header_token) => {
                    assert_eq!(header1_key, header_token.key.to_string());

                    assert_eq!(header1_value, header_token.value.to_str().unwrap());
                }

                RequestToken::Body(body_token) => {
                    assert!(body_token.is_some());

                    let body_inner = body_token.expect("body to be defined");

                    assert_eq!(body_input, body_inner);
                }

                RequestToken::HttpVersion(version_token) => {
                    assert_eq!(version_token, http::version::Version::HTTP_2);
                }
            }
        }
    }

    #[test]
    fn it_should_be_able_to_parse_multiple_requests() {
        let input = r"
GET https://mhouge.dk/ HTTP/0.9
x-test-header: test value

###

GET https://mhouge.dk/ HTTP/1.0
x-test-header: test value

###

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

        assert_eq!(25, tokens.len());

        let mut request_index: u8 = 0;
        for token in tokens {
            match token {
                RequestToken::Method(method_token) => {
                    assert_eq!("GET", method_token.as_str());
                    request_index += 1;
                }

                RequestToken::Uri(uri_token) => {
                    assert_eq!("https://mhouge.dk/", uri_token.to_string());
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

                RequestToken::HttpVersion(version_token) => match request_index {
                    1 => assert_eq!(version_token, http::Version::HTTP_09),
                    2 => assert_eq!(version_token, http::Version::HTTP_10),
                    3 => assert_eq!(version_token, http::Version::HTTP_11),
                    4 => assert_eq!(version_token, http::Version::HTTP_2),
                    5 => assert_eq!(version_token, http::Version::HTTP_3),
                    _ => panic!("this case should never hit"),
                },
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
    pub http_version: Option<http::version::Version>,
}

#[derive(Default)]
struct PartialHittRequest {
    method: Option<http::method::Method>,
    uri: Option<http::uri::Uri>,
    headers: http::HeaderMap,
    body: Option<String>,
    http_version: Option<http::version::Version>,
}

impl PartialHittRequest {
    #[inline]
    fn build(self) -> Result<HittRequest, RequestParseError> {
        match self.method {
            Some(method) => match self.uri {
                Some(uri) => Ok(HittRequest {
                    method,
                    uri,
                    headers: self.headers,
                    body: self.body,
                    http_version: self.http_version,
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

    let mut partial_request = PartialHittRequest::default();

    for token in tokens {
        match token {
            RequestToken::Method(method) => {
                if partial_request.method.is_some() {
                    requests.push(partial_request.build()?);

                    partial_request = PartialHittRequest::default();
                }

                partial_request.method = Some(method);
            }

            RequestToken::Uri(uri) => {
                partial_request.uri = Some(uri);
            }

            RequestToken::Header(header) => {
                partial_request.headers.insert(header.key, header.value);
            }

            RequestToken::Body(body) => {
                partial_request.body = body;

                requests.push(partial_request.build()?);

                partial_request = PartialHittRequest::default();
            }

            RequestToken::HttpVersion(version_token) => {
                partial_request.http_version = Some(version_token);
            }
        };
    }

    if partial_request.method.is_some() {
        requests.push(partial_request.build()?);
    };

    Ok(requests)
}

#[cfg(test)]
mod test_parse_requests {
    use core::str::FromStr;

    use crate::parse_requests;

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    #[test]
    fn it_should_parse_http_method_correctly() {
        let url = "https://mhouge.dk";

        for method in &HTTP_METHODS {
            let expected_method = http::Method::from_str(method).expect("m is a valid method");

            let parsed_requests =
                parse_requests(&format!("{method} {url}")).expect("request should be valid");

            assert!(parsed_requests.len() == 1);

            let first_request = &parsed_requests[0];

            assert_eq!(expected_method, first_request.method);

            let expected_uri = url.parse::<http::uri::Uri>().expect("url should be valid");

            assert_eq!(expected_uri, first_request.uri);

            assert_eq!(0, first_request.headers.len());

            assert_eq!(None, first_request.body);
        }
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

        assert_eq!(header1_value, header1_output.to_str().unwrap());

        assert!(request.http_version.is_none());
    }

    #[test]
    fn it_should_be_able_to_parse_multiple_requests() {
        let input = r"
GET https://mhouge.dk/ HTTP/0.9

###

GET https://mhouge.dk/ HTTP/1.0

###

GET https://mhouge.dk/ HTTP/1.1

###

GET https://mhouge.dk/ HTTP/2

###

GET https://mhouge.dk/ HTTP/3

###
";

        let requests = parse_requests(input).expect("to get a list of requests");

        assert_eq!(5, requests.len());

        for (request_index, request) in requests.iter().enumerate() {
            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");

            match request_index {
                0 => assert_eq!(http_version, http::Version::HTTP_09),
                1 => assert_eq!(http_version, http::Version::HTTP_10),
                2 => assert_eq!(http_version, http::Version::HTTP_11),
                3 => assert_eq!(http_version, http::Version::HTTP_2),
                4 => assert_eq!(http_version, http::Version::HTTP_3),
                _ => panic!("this case should never hit"),
            }
        }
    }
}
