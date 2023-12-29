use error::RequestParseError;
use header::{parse_header, HeaderToken};
use method::parse_method_input;
use uri::parse_uri_input;
use variables::{parse_variable, parse_variable_declaration};
use version::parse_http_version;

pub mod error;
mod header;
mod method;
mod uri;
mod variables;
mod version;

#[derive(Copy, Clone, PartialEq)]
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
fn to_enum_chars(input: &str) -> core::iter::Enumerate<core::str::Chars> {
    input.chars().enumerate()
}

#[inline]
fn tokenize(
    buffer: &str,
    input_variables: &std::collections::HashMap<String, String>,
) -> Result<Vec<RequestToken>, RequestParseError> {
    let mut tokens: Vec<RequestToken> = Vec::new();

    let mut parser_mode = ParserMode::Request;

    let mut body_parts: Vec<String> = Vec::new();

    let mut vars = input_variables.to_owned();

    for line in buffer.lines() {
        let trimmed_line = line.trim();

        // check if line is comment (#) OR requests break (###)
        if trimmed_line.starts_with('#') {
            if trimmed_line.starts_with("###") && parser_mode != ParserMode::Request {
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

        match parser_mode {
            ParserMode::Request => {
                if trimmed_line.starts_with('@') {
                    let mut chrs = to_enum_chars(trimmed_line);

                    // move forward once since we don't care about the '@'
                    chrs.next();

                    if let Some((name, value)) = parse_variable_declaration(&mut chrs) {
                        vars.insert(name, value);
                        continue;
                    }
                }

                if !trimmed_line.is_empty() {
                    let mut chrs = to_enum_chars(trimmed_line);
                    let method = parse_method_input(&mut chrs, &vars)?;

                    tokens.push(RequestToken::Method(method));

                    let uri = parse_uri_input(&mut chrs, &vars)?;

                    tokens.push(RequestToken::Uri(uri));

                    if let Some(http_version) = parse_http_version(&mut chrs, &vars) {
                        tokens.push(RequestToken::HttpVersion(http_version));
                    }

                    parser_mode = ParserMode::Headers;
                }
            }

            ParserMode::Headers => {
                if trimmed_line.is_empty() {
                    parser_mode = ParserMode::Body;
                } else if let Some(header_token) =
                    parse_header(&mut to_enum_chars(trimmed_line), &vars)?
                {
                    tokens.push(RequestToken::Header(header_token));
                }
            }

            ParserMode::Body => {
                let mut current_line = String::new();
                let mut chars = to_enum_chars(line);

                while let Some((_, ch)) = chars.next() {
                    if ch == '{' {
                        // FIXME: remove cloning of enumerator
                        if let Some((var, jumps)) = parse_variable(&mut chars.clone()) {
                            if let Some(variable_value) = vars.get(&var) {
                                current_line.push_str(variable_value);

                                for _ in 0..jumps {
                                    chars.next();
                                }

                                continue;
                            }

                            return Err(RequestParseError::VariableNotFound(var));
                        }
                    }

                    current_line.push(ch);
                }

                body_parts.push(current_line);
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
    use once_cell::sync::Lazy;

    use crate::{error::RequestParseError, tokenize, RequestToken};

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

    fn assert_method_token(token: &RequestToken, method: &http::Method) {
        assert!(matches!(token, RequestToken::Method(m) if m == method));
    }

    fn assert_uri_token(token: &RequestToken, uri: &str) {
        assert!(matches!(token , RequestToken::Uri(u) if u == uri));
    }

    fn assert_http_version_token(token: &RequestToken, version: http::Version) {
        assert!(matches!(token, RequestToken::HttpVersion(v) if v == &version));
    }

    fn assert_header_token(token: &RequestToken, key: &str, value: &str) {
        assert!(matches!(token , RequestToken::Header(h) if h.key  == key));

        assert!(matches!(token, RequestToken::Header(h) if h.value == value));
    }

    fn assert_body_token(token: &RequestToken, body: &Option<String>) {
        assert!(matches!(token, RequestToken::Body(b) if b == body));
    }

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

        let tokens =
            tokenize(&input_request, &EMPTY_VARS).expect("it to return Result<Vec<RequestToken>>");

        assert_eq!(tokens.len(), 5);

        for token in tokens {
            match token {
                RequestToken::Uri(uri_token) => assert_eq!(uri_input, uri_token.to_string()),
                RequestToken::Method(method_token) => {
                    assert_eq!(method_input, method_token.as_str());
                }
                RequestToken::Header(header_token) => {
                    assert_eq!(header1_key, header_token.key.to_string());

                    assert_eq!(
                        header1_value,
                        header_token
                            .value
                            .to_str()
                            .expect("value to be a valid str")
                    );
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
        let input = "
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
        let tokens = tokenize(input, &EMPTY_VARS).expect("it to return a list of tokens");

        assert_eq!(25, tokens.len());

        {
            let method_token = tokens.first().expect("it to be a method token");
            assert_method_token(method_token, &http::Method::GET);

            let uri_token = tokens.get(1).expect("it to be an uri token");
            assert_uri_token(uri_token, "https://mhouge.dk/");

            let version_token = tokens.get(2).expect("it to be a version token");
            assert_http_version_token(version_token, http::Version::HTTP_09);

            let header_token = tokens.get(3).expect("it to be a header token");
            assert_header_token(header_token, "x-test-header", "test value");

            let body_token = tokens.get(4).expect("it to be a body token");
            assert_body_token(body_token, &None);
        };

        {
            let method_token = tokens.get(5).expect("it to be a method token");
            assert_method_token(method_token, &http::Method::GET);

            let uri_token = tokens.get(6).expect("it to be an uri token");
            assert_uri_token(uri_token, "https://mhouge.dk/");

            let version_token = tokens.get(7).expect("it to be a version token");
            assert_http_version_token(version_token, http::Version::HTTP_10);

            let header_token = tokens.get(8).expect("it to be a header token");
            assert_header_token(header_token, "x-test-header", "test value");

            let body_token = tokens.get(9).expect("it to be a body token");
            assert_body_token(body_token, &None);
        };

        {
            let method_token = tokens.get(10).expect("it to be a method token");
            assert_method_token(method_token, &http::Method::GET);

            let uri_token = tokens.get(11).expect("it to be an uri token");
            assert_uri_token(uri_token, "https://mhouge.dk/");

            let version_token = tokens.get(12).expect("it to be a version token");
            assert_http_version_token(version_token, http::Version::HTTP_11);

            let header_token = tokens.get(13).expect("it to be a header token");
            assert_header_token(header_token, "x-test-header", "test value");

            let body_token = tokens.get(14).expect("it to be a body token");
            assert_body_token(body_token, &None);
        };

        {
            let method_token = tokens.get(15).expect("it to be a method token");
            assert_method_token(method_token, &http::Method::GET);

            let uri_token = tokens.get(16).expect("it to be an uri token");
            assert_uri_token(uri_token, "https://mhouge.dk/");

            let version_token = tokens.get(17).expect("it to be a version token");
            assert_http_version_token(version_token, http::Version::HTTP_2);

            let header_token = tokens.get(18).expect("it to be a header token");
            assert_header_token(header_token, "x-test-header", "test value");

            let body_token = tokens.get(19).expect("it to be a body token");
            assert_body_token(body_token, &None);
        };

        {
            let method_token = tokens.get(20).expect("it to be a method token");
            assert_method_token(method_token, &http::Method::GET);

            let uri_token = tokens.get(21).expect("it to be an uri token");
            assert_uri_token(uri_token, "https://mhouge.dk/");

            let version_token = tokens.get(22).expect("it to be a version token");
            assert_http_version_token(version_token, http::Version::HTTP_3);

            let header_token = tokens.get(23).expect("it to be a header token");
            assert_header_token(header_token, "x-test-header", "test value");

            let body_token = tokens.get(24).expect("it to be a body token");
            assert_body_token(body_token, &None);
        };
    }

    #[test]
    fn it_should_ignore_comments() {
        let input = "
// comment 1
# comment 2

DELETE https://mhouge.dk/";

        let tokens = tokenize(input, &EMPTY_VARS).expect("it to parse succesfully");

        assert_eq!(tokens.len(), 2);

        let method_token = tokens.first().expect("it to be some");

        assert!(
            matches!(method_token, RequestToken::Method(m) if m == http::method::Method::DELETE)
        );

        let uri_token = tokens.get(1).expect("it to be Some");

        let expected_uri = "https://mhouge.dk/";

        assert!(matches!(uri_token, RequestToken::Uri(uri) if uri == expected_uri));
    }

    #[test]
    fn it_should_support_variables() {
        {
            let input = "
@method = GET
@host = https://mhouge.dk
@path = /api
@query_value = mads@mhouge.dk
@body_input  = { \"key\": \"value\" }

{{method}} {{host}}{{path}}?email={{query_value}}

{{ body_input }}";

            let tokens = tokenize(input, &EMPTY_VARS).expect("it to tokenize successfully");

            assert_eq!(tokens.len(), 3);

            let method_token = tokens.first().expect("it to be some");

            assert!(
                matches!(method_token, RequestToken::Method(m) if m == http::method::Method::GET)
            );

            let uri_token = tokens.get(1).expect("it to be Some");

            let expected_uri = "https://mhouge.dk/api?email=mads@mhouge.dk";

            assert!(matches!(uri_token, RequestToken::Uri(uri) if uri == expected_uri));

            let body_token = tokens.get(2).expect("it to be set");

            let expected_body_value = "{ \"key\": \"value\" }";

            assert!(matches!(
                body_token,
                RequestToken::Body(value)
                if value.clone().expect( "value to exist") == expected_body_value
            ));
        };

        {
            let input = "
GET https://mhouge.dk/

{{ body_input }}";

            let tokens = tokenize(input, &EMPTY_VARS).expect_err("it to return an error");

            assert!(matches!(
                tokens,
                RequestParseError::VariableNotFound(var)
                if var == "body_input"
            ));
        }
    }

    #[test]
    fn it_should_support_input_variables() {
        let vars = std::collections::HashMap::from([
            ("method".to_owned(), "GET".to_owned()),
            ("host".to_owned(), "https://mhouge.dk".to_owned()),
            ("path".to_owned(), "/api".to_owned()),
            ("query_value".to_owned(), "mads@mhouge.dk".to_owned()),
            ("body_input".to_owned(), "{ \"key\": \"value\" }".to_owned()),
        ]);

        let input = "
{{method}} {{host}}{{path}}?email={{query_value}}

{{ body_input }}";

        let tokens = tokenize(input, &vars).expect("it to tokenize successfully");

        assert_eq!(tokens.len(), 3);

        let method_token = tokens.first().expect("it to return token");

        assert!(
            matches!(method_token, RequestToken::Method(method) if method == http::Method::GET)
        );

        let expected_uri = "https://mhouge.dk/api?email=mads@mhouge.dk";

        let uri_token = tokens.get(1).expect("it to return an uri");

        assert!(matches!(uri_token, RequestToken::Uri(uri) if uri == expected_uri));

        let body_token = tokens.get(2).expect("it to return a token");

        let expected_body = "{ \"key\": \"value\" }";

        assert!(
            matches!(body_token, RequestToken::Body(Some(output_body)) if output_body == expected_body)
        );
    }

    #[test]
    fn input_variables_should_be_overwritten_by_local_variables() {
        let vars = std::collections::HashMap::from([("method".to_owned(), "PUT".to_owned())]);

        let input = "
@method = POST

{{method}} https://mhouge.dk/";

        let tokens = tokenize(input, &vars).expect("it to parse successfully");

        assert_eq!(tokens.len(), 2);

        let method_token = tokens.first().expect("it to return token");

        assert!(
            matches!(method_token, RequestToken::Method(method) if method == http::Method::POST)
        );

        let uri_token = tokens.get(1).expect("it to return token");

        assert!(matches!(uri_token, RequestToken::Uri(uri) if uri == "https://mhouge.dk/"));
    }

    #[test]
    fn it_should_support_multiple_requests() {
        let input = "
CONNECT https://mhouge.dk/?q=mads HTTP/2
key: value

mads was here
###

PUT https://mhouge.dk/ HTTP/3
x-test-header:{{test value

mads
was
here
###



";

        let output = tokenize(input, &EMPTY_VARS).expect("it to parse succesfully");

        assert_eq!(output.len(), 10);

        {
            let method = output.first().expect("it to be a method token");
            assert_method_token(method, &http::Method::CONNECT);

            let uri = output.get(1).expect("it to be a uri token");
            assert_uri_token(uri, "https://mhouge.dk/?q=mads");

            let version = output.get(2).expect("it to be a version token");
            assert_http_version_token(version, http::Version::HTTP_2);

            let header = output.get(3).expect("it to be a header token");
            assert_header_token(header, "key", "value");

            let body = output.get(4).expect("it to be a body token");
            assert_body_token(body, &Some("mads was here".to_owned()));
        };

        {
            let method = output.get(5).expect("it to be a method token");
            assert_method_token(method, &http::Method::PUT);

            let uri = output.get(6).expect("it to be a uri token");
            assert_uri_token(uri, "https://mhouge.dk/");

            let version = output.get(7).expect("it to be a version token");
            assert_http_version_token(version, http::Version::HTTP_3);

            let header = output.get(8).expect("it to be a header token");
            assert_header_token(header, "x-test-header", "{{test value");

            let body = output.get(9).expect("it to be a body token");
            assert_body_token(body, &Some("mads\nwas\nhere".to_owned()));
        };
    }

    #[test]
    fn it_should_ignore_triple_hashtag_when_in_parsermode_request() {
        let input = "
###

###

###

OPTIONS https://mhouge.dk/
###
###
###

HEAD https://mhouge.dk/blog/";

        let output = tokenize(input, &EMPTY_VARS).expect("it to parse");

        assert_eq!(output.len(), 5);

        {
            let method = output.first().expect("it to return a method token");
            assert_method_token(method, &http::method::Method::OPTIONS);

            let uri = output.get(1).expect("it to return a uri token");
            assert_uri_token(uri, "https://mhouge.dk/");

            let body = output.get(2).expect("it to be an empty body token");
            assert_body_token(body, &None);
        };

        {
            let method = output.get(3).expect("it to return a method token");
            assert_method_token(method, &http::method::Method::HEAD);

            let uri = output.get(4).expect("it to return a uri token");
            assert_uri_token(uri, "https://mhouge.dk/blog/");
        };
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

#[cfg(test)]
mod test_partial_http_request {
    use core::str::FromStr;

    use http::{HeaderMap, Uri};

    use crate::{error::RequestParseError, PartialHittRequest};

    #[test]
    fn build_should_reject_if_no_uri() {
        let request = PartialHittRequest {
            uri: None,
            method: Some(http::Method::GET),
            http_version: None,
            headers: HeaderMap::default(),
            body: None,
        }
        .build()
        .expect_err("it to raise RequestParseError::MissingUri");

        assert!(matches!(request, RequestParseError::MissingUri));
    }

    #[test]
    fn build_should_reject_if_no_method() {
        let uri = Uri::from_str("https://mhouge.dk/").expect("it to be a valid url");

        let request = PartialHittRequest {
            uri: Some(uri),
            method: None,
            http_version: None,
            headers: HeaderMap::default(),
            body: None,
        }
        .build()
        .expect_err("it to raise RequestParseError::MissingMethod");

        assert!(matches!(request, RequestParseError::MissingMethod));
    }
}

#[inline]
pub fn parse_requests(
    buffer: &str,
    input_variables: &std::collections::HashMap<String, String>,
) -> Result<Vec<HittRequest>, RequestParseError> {
    let mut requests = Vec::new();

    let tokens = tokenize(buffer, input_variables)?;

    let mut partial_request = PartialHittRequest::default();

    for token in tokens {
        match token {
            RequestToken::Method(method) => {
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

    use once_cell::sync::Lazy;

    use crate::{error::RequestParseError, parse_requests};

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

    #[test]
    fn it_should_parse_http_method_correctly() {
        let url = "https://mhouge.dk";

        for method in &HTTP_METHODS {
            let expected_method = http::Method::from_str(method).expect("m is a valid method");

            let input = format!("{method} {url}");

            let parsed_requests =
                parse_requests(&input, &EMPTY_VARS).expect("request should be valid");

            assert!(parsed_requests.len() == 1);

            let first_request = parsed_requests.first().expect("it to be a request");

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

        let result =
            parse_requests(&input_request, &EMPTY_VARS).expect("it to return a list of requests");

        assert!(result.len() == 1);

        let request = result.first().expect("request len to be 1");

        assert_eq!(method_input, request.method.as_str());

        assert_eq!(uri_input, request.uri.to_string());

        let body_inner = request.body.clone().expect("body to be defined");

        assert_eq!(body_inner, body_input);

        assert_eq!(1, request.headers.len());

        let header1_output = request
            .headers
            .get(header1_key)
            .expect("header1_key to exist");

        assert_eq!(
            header1_value,
            header1_output.to_str().expect("it to be a valid header")
        );

        assert!(request.http_version.is_none());
    }

    #[test]
    fn it_should_be_able_to_parse_multiple_requests() {
        let input = "
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

        let requests = parse_requests(input, &EMPTY_VARS).expect("to get a list of requests");

        assert_eq!(5, requests.len());

        {
            let request = requests.first().expect("it to be exist");

            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");
            assert_eq!(http_version, http::Version::HTTP_09);
        };

        {
            let request = requests.get(1).expect("it to be exist");

            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");
            assert_eq!(http_version, http::Version::HTTP_10);
        };

        {
            let request = requests.get(2).expect("it to be exist");

            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");
            assert_eq!(http_version, http::Version::HTTP_11);
        };

        {
            let request = requests.get(3).expect("it to be exist");

            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");
            assert_eq!(http_version, http::Version::HTTP_2);
        };

        {
            let request = requests.get(4).expect("it to be exist");

            assert_eq!("GET", request.method.as_str());

            assert_eq!("https://mhouge.dk/", request.uri.to_string());

            assert!(request.headers.is_empty());

            assert!(request.body.is_none());

            let http_version = request.http_version.expect("http_version to be defined");
            assert_eq!(http_version, http::Version::HTTP_3);
        };
    }

    #[test]
    fn it_should_support_variables() {
        let input = "
@method = GET
@host = https://mhouge.dk
@path = /api
@query_value = mads@mhouge.dk
@body_input  = { \"key\": \"value\" }

{{method}} {{host}}{{path}}?email={{query_value}}

{{ body_input }}";

        let requests = parse_requests(input, &EMPTY_VARS).expect("to get a list of requests");

        assert_eq!(requests.len(), 1);

        let request = requests.first().expect("it to have 1 request");

        assert_eq!(request.method, http::method::Method::GET);

        assert_eq!(request.uri, "https://mhouge.dk/api?email=mads@mhouge.dk");

        assert_eq!(
            "{ \"key\": \"value\" }",
            request.body.clone().expect("body to be set"),
        );
    }

    #[test]
    fn it_should_support_variable_input() {
        {
            let mut vars = std::collections::HashMap::from([
                ("method".to_owned(), "GET".to_owned()),
                ("host".to_owned(), "https://mhouge.dk".to_owned()),
                ("path".to_owned(), "/api".to_owned()),
                ("query_value".to_owned(), "mads@mhouge.dk".to_owned()),
                ("body_input".to_owned(), "{ \"key\": \"value\" }".to_owned()),
            ]);

            let input = "
{{method}} {{host}}{{path}}?email={{query_value}}
{{header_name}}: {{header_value}}

{{ body_input }}";

            for i in u8::MIN..u8::MAX {
                let header_name = format!("mads-was-here{i}");
                let header_value = format!("or was i{i}?");

                vars.insert("header_name".to_owned(), header_name.clone());
                vars.insert("header_value".to_owned(), header_value.clone());

                let requests = parse_requests(input, &vars).expect("to get a list of requests");

                assert_eq!(requests.len(), 1);

                let request = requests.first().expect("it to have 1 request");

                assert_eq!(request.method, http::method::Method::GET);

                assert_eq!(request.uri, "https://mhouge.dk/api?email=mads@mhouge.dk");

                assert_eq!(request.headers.len(), 1);

                let result_header_value = request.headers.get(header_name).expect("it to exist");

                assert_eq!(
                    header_value,
                    result_header_value
                        .to_str()
                        .expect("it to be a valid string"),
                );

                assert_eq!(
                    "{ \"key\": \"value\" }",
                    request.body.clone().expect("body to be set"),
                );
            }
        }

        {
            let input = "
GET https://mhouge.dk/

{{ body_input }}";

            let request = parse_requests(input, &EMPTY_VARS).expect_err("it to return an error");

            assert!(matches!(
                request,
                RequestParseError::VariableNotFound(var)
                if var == "body_input"
            ));
        }
    }

    #[test]
    fn input_variables_should_be_overwritten_by_local_variables() {
        let vars = std::collections::HashMap::from([("method".to_owned(), "PUT".to_owned())]);

        let input = "
@method = POST

{{method}} https://mhouge.dk/";

        let requests = parse_requests(input, &vars).expect("it to parse successfully");

        assert_eq!(requests.len(), 1);

        let request = requests.first().expect("it to exist");

        assert_eq!(request.method, http::method::Method::POST);

        assert_eq!(request.uri, "https://mhouge.dk/");

        assert_eq!(request.headers.len(), 0);
    }

    #[test]
    fn it_should_ignore_comments() {
        let input = "
// comment 1
# comment 2

DELETE https://mhouge.dk/";

        let requests = parse_requests(input, &EMPTY_VARS).expect("it to parse succesfully");

        assert_eq!(requests.len(), 1);

        let request = requests.first().expect("it to exist");

        assert_eq!(request.method, http::method::Method::DELETE);

        let expected_uri = "https://mhouge.dk/";

        assert_eq!(request.uri, expected_uri);

        assert!(request.headers.is_empty());

        assert!(request.body.is_none());
    }
}
