use core::str::FromStr;

use crate::{error::RequestParseError, RequestToken};

impl From<http::method::Method> for RequestToken {
    #[inline]
    fn from(value: http::method::Method) -> Self {
        Self::Method(value)
    }
}

#[inline]
pub fn parse_method_input(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
) -> Result<http::method::Method, RequestParseError> {
    let mut method = String::new();

    for (_i, ch) in chars {
        if ch.is_whitespace() {
            if !method.is_empty() {
                break;
            }
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
    use crate::{method::parse_method_input, to_enum_chars};

    const HTTP_METHODS: [&str; 9] = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    #[test]
    fn it_should_accept_valid_methods() {
        for method_input in HTTP_METHODS {
            let input = format!("{method_input} https://mhouge.dk HTTP/2");

            let parsed_method = parse_method_input(&mut to_enum_chars(&input))
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }

    #[test]
    fn it_should_ignore_case() {
        for method_input in HTTP_METHODS {
            let input = format!("{} https://mhouge.dk HTTP/2", method_input.to_lowercase());

            let parsed_method = parse_method_input(&mut to_enum_chars(&input))
                .expect("it should return a valid method");

            assert_eq!(method_input, parsed_method.as_str());
        }
    }
}
