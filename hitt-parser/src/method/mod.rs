use core::str::FromStr;

use crate::{error::RequestParseError, variables::parse_variable, RequestToken};

impl From<http::method::Method> for RequestToken {
    #[inline]
    fn from(value: http::method::Method) -> Self {
        Self::Method(value)
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
                } else {
                    return Err(RequestParseError::VariableNotFound(var));
                }
            }
        }
        {
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

    use once_cell::sync::Lazy;

    use crate::{method::parse_method_input, to_enum_chars};

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

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
}
