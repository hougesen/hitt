use core::str::FromStr;

use crate::{error::RequestParseError, variables::parse_variable, RequestToken};

#[derive(Debug)]
pub struct HeaderToken {
    pub(super) key: http::HeaderName,
    pub(super) value: http::HeaderValue,
}

impl From<HeaderToken> for RequestToken {
    #[inline]
    fn from(value: HeaderToken) -> Self {
        Self::Header(value)
    }
}

#[inline]
pub fn parse_header(
    line: &mut core::iter::Enumerate<core::str::Chars>,
    vars: &std::collections::HashMap<String, String>,
) -> Result<Option<HeaderToken>, RequestParseError> {
    let mut key = String::new();
    let mut value = String::new();
    let mut is_key = true;

    while let Some((_index, ch)) = line.next() {
        if ch == ':' {
            if is_key {
                is_key = false;
            } else {
                value.push(ch);
            }
        } else if ch == '{' {
            // FIXME: remove cloning of enumerator
            if let Some((var, jumps)) = parse_variable(&mut line.clone()) {
                if let Some(variable_value) = vars.get(&var) {
                    if is_key {
                        key.push_str(variable_value);
                    } else {
                        value.push_str(variable_value);
                    }

                    for _ in 0..jumps {
                        line.next();
                    }
                } else {
                    return Err(RequestParseError::VariableNotFound(var));
                }
            } else {
                if is_key {
                    key.push(ch);
                }
                {
                    value.push(ch);
                }
            }
        } else if is_key {
            key.push(ch);
        } else if !(value.is_empty() && ch == ' ') {
            value.push(ch);
        }
    }

    if !key.is_empty() {
        return Ok(Some(HeaderToken {
            key: http::HeaderName::from_str(&key)
                .map_err(|_err| RequestParseError::InvalidHeaderName(key))?,
            value: http::HeaderValue::from_str(&value)
                .map_err(|_err| RequestParseError::InvalidHeaderValue(value))?,
        }));
    }
    Ok(None)
}

#[cfg(test)]
mod test_parse_header {
    use core::str::FromStr;

    use once_cell::sync::Lazy;

    use crate::{parse_header, to_enum_chars};

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

    #[test]
    fn it_should_return_valid_headers() {
        for i in i8::MIN..i8::MAX {
            let line = format!("header{i}: value{i}");

            let result = parse_header(&mut to_enum_chars(&line), &EMPTY_VARS)
                .expect("It should be able to parse valid headers")
                .expect("headers to be defined");

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
        let result = parse_header(&mut to_enum_chars(""), &EMPTY_VARS).expect("it to be parseable");

        assert!(result.is_none());
    }

    #[test]
    fn it_should_support_variables() {
        let mut vars = std::collections::HashMap::new();

        let open = "{{";
        let close = "}}";
        let mut extra_spaces = String::new();

        for i in i8::MIN..i8::MAX {
            let key = format!("key{i}");
            let value = format!("value{i}");

            vars.insert(key.clone(), i.to_string());
            vars.insert(value.clone(), i.to_string());

            let dynamic_key =
                format!("{open}{extra_spaces}{key}{extra_spaces}{close}:{extra_spaces}static");

            let dynamic_key_result = parse_header(&mut to_enum_chars(&dynamic_key), &vars)
                .expect("it to be parseable")
                .expect("it to return a header field");

            assert_eq!(dynamic_key_result.key.as_str(), i.to_string());
            assert_eq!(dynamic_key_result.value, "static",);

            let dynamic_value =
                format!("static:{extra_spaces}{open}{extra_spaces}{value}{extra_spaces}{close}");

            let dynamic_value_result = parse_header(&mut to_enum_chars(&dynamic_value), &vars)
                .expect("it to be parseable")
                .expect("it to return a header field");

            assert_eq!(dynamic_value_result.key.as_str(), "static");
            assert_eq!(dynamic_value_result.value, i.to_string());

            let dynamic_key_value =
                format!("{open}{extra_spaces}{key}{extra_spaces}{close}:{extra_spaces}{open}{extra_spaces}{value}{extra_spaces}{close}");

            let dynamic_key_value_result =
                parse_header(&mut to_enum_chars(&dynamic_key_value), &vars)
                    .expect("it to be parseable")
                    .expect("it to return a header field");

            assert_eq!(dynamic_key_value_result.key.as_str(), i.to_string());
            assert_eq!(dynamic_key_value_result.value, i.to_string());

            extra_spaces.push(' ');
        }
    }
}
