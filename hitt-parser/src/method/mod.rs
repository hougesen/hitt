use core::str::FromStr;

use crate::{RequestToken, error::RequestParseError, variables::parse_variable};

impl From<http::method::Method> for RequestToken {
    #[inline]
    fn from(value: http::method::Method) -> Self {
        Self::Method(value)
    }
}

#[cfg(test)]
mod test_from_method_from_request_token {
    #[test]
    fn it_should_wrap_method() {
        let methods = [
            http::method::Method::GET,
            http::method::Method::OPTIONS,
            http::method::Method::POST,
            http::method::Method::PUT,
            http::method::Method::PATCH,
            http::method::Method::DELETE,
            http::method::Method::TRACE,
            http::method::Method::HEAD,
            http::method::Method::CONNECT,
        ];

        for method in methods {
            let token = crate::RequestToken::from(method.clone());

            assert!(matches!(token, crate::RequestToken::Method(m) if m == method));
        }
    }
}

#[inline]
pub fn parse_method_input(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
    vars: &std::collections::HashMap<String, String>,
) -> Result<http::method::Method, RequestParseError> {
    let mut method = String::new();

    while let Some((_, ch)) = chars.next() {
        if ch.is_whitespace() {
            if !method.is_empty() {
                break;
            }
        } else if ch == '{' {
            // FIXME: remove clone
            if let Some((var, jumps)) = parse_variable(&mut chars.clone()) {
                if let Some(var_value) = vars.get(&var) {
                    method.push_str(var_value);

                    for _ in 0..jumps {
                        chars.next();
                    }

                    continue;
                }

                return Err(RequestParseError::VariableNotFound(var));
            }

            method.push(ch);
        } else {
            method.push(ch);
        }
    }

    let uppercase_method = method.to_uppercase();

    http::method::Method::from_str(&uppercase_method)
        .map_err(|_err| RequestParseError::InvalidHttpMethod(uppercase_method))
}

#[cfg(test)]
mod test_parse_method_input {
    use core::str::FromStr;

    use crate::{error::RequestParseError, method::parse_method_input, to_enum_chars};

    static EMPTY_VARS: std::sync::LazyLock<std::collections::HashMap<String, String>> =
        std::sync::LazyLock::new(std::collections::HashMap::new);

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    #[test]
    fn it_should_accept_valid_methods() {
        for method_input in HTTP_METHODS {
            let input = format!("{method_input} https://mhouge.dk HTTP/2");

            let parsed_method = parse_method_input(&mut to_enum_chars(&input), &EMPTY_VARS)
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }

    #[test]
    fn it_should_ignore_case() {
        for method_input in HTTP_METHODS {
            let input = format!("{} https://mhouge.dk HTTP/2", method_input.to_lowercase());

            let parsed_method = parse_method_input(&mut to_enum_chars(&input), &EMPTY_VARS)
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }

    #[test]
    fn it_should_support_variables() {
        let mut vars = std::collections::HashMap::new();

        {
            let error = parse_method_input(&mut to_enum_chars("{method"), &EMPTY_VARS)
                .expect_err("invalid method");
            assert_eq!("invalid HTTP method '{METHOD'", error.to_string());
            assert!(matches!(error, RequestParseError::InvalidHttpMethod(m) if m == "{METHOD"));
        };

        {
            #[allow(clippy::literal_string_with_formatting_args)]
            let error = parse_method_input(&mut to_enum_chars("{method}"), &EMPTY_VARS)
                .expect_err("invalid method");
            assert_eq!("invalid HTTP method '{METHOD}'", error.to_string());
            assert!(matches!(error, RequestParseError::InvalidHttpMethod(m) if m == "{METHOD}"));
        };

        for method in HTTP_METHODS {
            vars.insert("method".to_owned(), method.to_owned());

            let expected_method =
                http::method::Method::from_str(method).expect("it should return a valid method");

            assert_eq!(
                expected_method,
                parse_method_input(&mut to_enum_chars("{{method}}"), &vars)
                    .expect("it should return a valid method")
            );

            assert_eq!(
                expected_method,
                parse_method_input(&mut to_enum_chars("{{  method  }}"), &vars)
                    .expect("it should return a valid method")
            );
        }
    }

    #[test]
    fn it_should_require_method() {
        let input = "   ";

        let output = parse_method_input(&mut to_enum_chars(input), &EMPTY_VARS)
            .expect_err("it to return a RequestParseError");

        assert_eq!(output.to_string(), "invalid HTTP method ''");

        assert!(matches!(output, RequestParseError::InvalidHttpMethod(m) if m.is_empty()));
    }

    #[test]
    fn it_should_raise_if_variable_not_found() {
        {
            let input = "{{}}";

            let output = parse_method_input(&mut to_enum_chars(input), &EMPTY_VARS);

            assert!(matches!(output, Err(RequestParseError::InvalidHttpMethod(m)) if m == input));
        };

        {
            let input = "{{method}}";

            let output = parse_method_input(&mut to_enum_chars(input), &EMPTY_VARS);

            assert!(
                matches!(output, Err(RequestParseError::VariableNotFound(var)) if var == "method")
            );
        }
    }
}
