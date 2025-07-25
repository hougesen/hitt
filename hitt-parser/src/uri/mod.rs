use core::str::FromStr;

use crate::{RequestToken, error::RequestParseError, variables::parse_variable};

impl From<http::uri::Uri> for RequestToken {
    #[inline]
    fn from(value: http::uri::Uri) -> Self {
        Self::Uri(value)
    }
}

#[cfg(test)]
mod test_from_uri_for_request_token {
    use crate::RequestToken;

    #[test]
    fn it_should_wrap_uri() {
        let uri = http::Uri::from_static("https://mhouge.dk/");

        let token = RequestToken::from(uri.clone());

        assert!(matches!(token, RequestToken::Uri(inner_uri) if inner_uri == uri));
    }
}

#[inline]
pub fn parse_uri_input(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
    vars: &std::collections::HashMap<String, String>,
) -> Result<http::uri::Uri, RequestParseError> {
    let mut uri = String::new();

    while let Some((_, ch)) = chars.next() {
        if ch.is_whitespace() {
            if !uri.is_empty() {
                break;
            }
        } else if ch == '{' {
            // FIXME: remove clone
            if let Some((var, jumps)) = parse_variable(&mut chars.clone()) {
                if let Some(var_value) = vars.get(&var) {
                    uri.push_str(var_value);

                    for _ in 0..jumps {
                        chars.next();
                    }
                } else {
                    return Err(RequestParseError::VariableNotFound(var));
                }
            } else {
                uri.push(ch);
            }
        } else {
            uri.push(ch);
        }
    }

    http::uri::Uri::from_str(&uri).map_err(|_err| RequestParseError::InvalidUri(uri))
}

#[cfg(test)]
mod test_parse_uri_input {
    use crate::{error::RequestParseError, to_enum_chars, uri::parse_uri_input};

    static EMPTY_VARS: std::sync::LazyLock<std::collections::HashMap<String, String>> =
        std::sync::LazyLock::new(std::collections::HashMap::new);

    #[test]
    fn it_should_be_able_to_parse_uris() {
        let input_uris = [
            "https://mhouge.dk/",
            "https://goout.dk/",
            "https://mhouge.dk?key=value",
        ];

        for input_uri in input_uris {
            let input = format!("{input_uri} HTTP/2");

            let result = parse_uri_input(&mut to_enum_chars(&input), &EMPTY_VARS);

            let output_uri = result.expect("it to parse uri correctly");

            assert_eq!(input_uri, output_uri);
        }
    }

    #[test]
    fn it_should_ignore_leading_spaces() {
        let input_uri = "https://mhouge.dk/";

        let input = format!("         {input_uri} HTTP/2.0");

        let result = parse_uri_input(&mut to_enum_chars(&input), &EMPTY_VARS)
            .expect("it should return a valid uri");

        assert_eq!(result.to_string(), input_uri);
    }

    #[test]
    fn it_should_reject_invalid_uris() {
        let invalid_uris = ["m:a:d:s"];

        for invalid_uri in invalid_uris {
            let input = format!("{invalid_uri} HTTP/2");

            let error = parse_uri_input(&mut to_enum_chars(&input), &EMPTY_VARS)
                .expect_err("it should return an error");

            assert_eq!(format!("invalid uri '{invalid_uri}'"), error.to_string());
            assert!(matches!(error, RequestParseError::InvalidUri(u) if u == invalid_uri));
        }
    }

    #[test]
    fn it_should_support_query_parameters() {
        let input_uri = "https://mhouge.dk/";

        for i in i8::MIN..i8::MAX {
            let input = format!("{input_uri}?key{i}=value{i}");

            let result = parse_uri_input(&mut to_enum_chars(&input), &EMPTY_VARS)
                .expect("it should return a valid uri");

            assert_eq!(result.to_string(), format!("{input_uri}?key{i}=value{i}"));
        }
    }

    #[test]
    fn it_should_support_variables() {
        let mut vars = std::collections::HashMap::new();

        let input_uri = "https://mhouge.dk/";

        let variable_open = "{{";
        let variable_close = "}}";

        for i in i8::MIN..i8::MAX {
            vars.insert(format!("i{i}"), i.to_string());

            let input = format!("{input_uri}?key={variable_open}i{i}{variable_close}");

            let result = parse_uri_input(&mut to_enum_chars(&input), &vars)
                .expect("it should return a valid uri");

            // with actual variable
            assert_eq!(result.to_string(), format!("{input_uri}?key={i}"));
        }

        let open_bracket = '{';
        let close_bracket = '}';

        let bad_variable_input = [
            format!("{open_bracket}val"),
            format!("{open_bracket}val{close_bracket}"),
            format!("{open_bracket}val{close_bracket}"),
            format!("{open_bracket}val{close_bracket}{open_bracket}"),
            format!("{open_bracket}val{close_bracket}{close_bracket}"),
        ];

        for input in bad_variable_input {
            let uri = format!("https://mhouge.dk/?key={input}");

            let result = parse_uri_input(&mut to_enum_chars(&uri), &EMPTY_VARS)
                .expect("it to parse as a valid uri");

            assert_eq!(result.to_string(), uri);
        }
    }

    #[test]
    fn it_should_raise_if_variable_isnt_found() {
        {
            let input = "{{host}}";

            let output = parse_uri_input(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("to to return a RequestParseError");

            assert_eq!("variable 'host' was used, but not set", output.to_string());

            assert!(matches!(
                output,
                RequestParseError::VariableNotFound(var)
                if var == "host"
            ));
        };

        {
            let input = "{{  host}}";

            let output = parse_uri_input(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("to to return a RequestParseError");

            assert_eq!("variable 'host' was used, but not set", output.to_string());

            assert!(matches!(
                output,
                RequestParseError::VariableNotFound(var)
                if var == "host"
            ));
        };

        {
            let input = "{{host  }}";

            let output = parse_uri_input(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("to to return a RequestParseError");

            assert_eq!("variable 'host' was used, but not set", output.to_string());

            assert!(matches!(
                output,
                RequestParseError::VariableNotFound(var)
                if var == "host"
            ));
        };

        {
            let input = "{{  host  }}";

            let output = parse_uri_input(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("to to return a RequestParseError");

            assert_eq!("variable 'host' was used, but not set", output.to_string());

            assert!(matches!(
                output,
                RequestParseError::VariableNotFound(var)
                if var == "host"
            ));
        }
    }
}
