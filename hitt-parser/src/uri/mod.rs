use core::str::FromStr;

use crate::{error::RequestParseError, variables::parse_variable, RequestToken};

impl From<http::uri::Uri> for RequestToken {
    #[inline]
    fn from(value: http::uri::Uri) -> Self {
        Self::Uri(value)
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
    use once_cell::sync::Lazy;

    use crate::{to_enum_chars, uri::parse_uri_input};

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

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

            parse_uri_input(&mut to_enum_chars(&input), &EMPTY_VARS)
                .expect_err("it should return an error");
        }
    }

    #[test]
    fn it_should_support_query_paramers() {
        let input_uri = "https://mhouge.dk/";

        for i in 0..1337 {
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

        for i in 0..1337 {
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
}
