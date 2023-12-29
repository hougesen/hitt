use core::str::FromStr;

use crate::{error::RequestParseError, variables::parse_variable, RequestToken};

#[derive(Debug)]
pub struct HeaderToken {
    pub key: http::HeaderName,
    pub value: http::HeaderValue,
}

impl From<HeaderToken> for RequestToken {
    #[inline]
    fn from(value: HeaderToken) -> Self {
        Self::Header(value)
    }
}

#[cfg(test)]
mod test_from_header_token_for_request_token {
    use core::str::FromStr;

    use crate::RequestToken;

    use super::HeaderToken;

    #[test]
    fn it_should_wrap() {
        let input_key = http::HeaderName::from_str("mads").expect("it to be valid HeaderName");
        let input_value = http::HeaderValue::from_str("mads").expect("it to be valid HeaderValue");

        let output = RequestToken::from(HeaderToken {
            key: input_key.clone(),
            value: input_value.clone(),
        });

        assert!(matches!(
            output,
            RequestToken::Header(HeaderToken { key, value })
            if key == input_key && value == input_value
        ));
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

    while let Some((_, ch)) = line.next() {
        if ch == ':' {
            if is_key {
                is_key = false;
            } else {
                value.push(ch);
            }

            continue;
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

                    continue;
                }

                return Err(RequestParseError::VariableNotFound(var));
            }

            if is_key {
                key.push(ch);
            } else {
                value.push(ch);
            }
        } else if is_key {
            key.push(ch);
        } else {
            value.push(ch);
        }
    }

    let trimmed_key = key.trim();

    let trimmed_value = value.trim();

    if !key.is_empty() {
        return Ok(Some(HeaderToken {
            key: http::HeaderName::from_str(trimmed_key)
                .map_err(|_err| RequestParseError::InvalidHeaderName(trimmed_key.to_owned()))?,
            value: http::HeaderValue::from_str(trimmed_value)
                .map_err(|_err| RequestParseError::InvalidHeaderValue(trimmed_value.to_owned()))?,
        }));
    }

    Ok(None)
}

#[cfg(test)]
mod test_parse_header {
    use core::str::FromStr;

    use once_cell::sync::Lazy;

    use crate::{error::RequestParseError, parse_header, to_enum_chars};

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

        {
            let input = "key===::value";

            let output = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to fail to parse");

            assert!(
                matches!(output, RequestParseError::InvalidHeaderName(name) if name == "key===")
            );
        };

        {
            let input = "key::v!###  `al\nue";

            let output = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to fail to parse");

            assert!(
                matches!(output, RequestParseError::InvalidHeaderValue(val) if val == ":v!###  `al\nue")
            );
        };

        {
            let input = "key::value";

            let output = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to parse")
                .expect("it to be some");

            assert_eq!(output.key, "key");
            assert_eq!(output.value, ":value");
        };
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

            {
                let input =
                    format!("{open}{extra_spaces}{key}{extra_spaces}{close}:{extra_spaces}static");

                let result = parse_header(&mut to_enum_chars(&input), &vars)
                    .expect("it to be parseable")
                    .expect("it to return a header field");

                assert_eq!(result.key.as_str(), i.to_string());
                assert_eq!(result.value, "static");
            };

            {
                let input = format!(
                    "static:{extra_spaces}{open}{extra_spaces}{value}{extra_spaces}{close}"
                );

                let result = parse_header(&mut to_enum_chars(&input), &vars)
                    .expect("it to be parseable")
                    .expect("it to return a header field");

                assert_eq!(result.key.as_str(), "static");
                assert_eq!(result.value, i.to_string());
            };

            {
                let input =
                format!("{open}{extra_spaces}{key}{extra_spaces}{close}:{extra_spaces}{open}{extra_spaces}{value}{extra_spaces}{close}");

                let result = parse_header(&mut to_enum_chars(&input), &vars)
                    .expect("it to be parseable")
                    .expect("it to return a header field");

                assert_eq!(result.key.as_str(), i.to_string());
                assert_eq!(result.value, i.to_string());
            };

            extra_spaces.push(' ');
        }
    }

    #[test]
    fn it_should_handle_bad_variables() {
        {
            let input = "{key:value";

            parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return an invalid error");
        };

        {
            let input = "{key }:value";

            parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return an invalid error");
        };

        {
            let input = "{key:value }}";

            parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return an invalid error");
        };

        {
            let input = "key:{value";

            let result = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to be parseable")
                .expect("it to return a header field");

            assert_eq!(result.key.as_str(), "key");
            assert_eq!(result.value, "{value");
        };

        {
            let input = "key{:value";

            parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return an invalid error");
        };

        {
            let input = "key{:value}}";

            parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return an invalid error");
        };
    }

    #[test]
    fn it_should_allow_spaces_in_header() {
        let f = "mads-was-here";
        let input = format!("     {f}    :     {f}     ");
        let result = parse_header(&mut to_enum_chars(&input), &EMPTY_VARS)
            .expect("it to be parseable")
            .expect("it to exist");

        assert_eq!(f.trim(), result.key);

        assert_eq!(f.trim(), result.value);
    }

    #[test]
    fn it_should_reject_if_variable_is_missing() {
        {
            let input = "{{key_var}}: value";

            let output = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return missing variable 'key_var'");

            assert!(matches!(output, RequestParseError::VariableNotFound(var) if var == "key_var"));
        };

        {
            let input = "key: {{value_var}}";

            let output = parse_header(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect_err("it to return missing variable 'value_var'");

            assert!(
                matches!(output, RequestParseError::VariableNotFound(var) if var == "value_var")
            );
        };
    }
}
