use core::str::FromStr;

use crate::{error::RequestParseError, RequestToken};

#[derive(Debug)]
pub(super) struct HeaderToken {
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
pub(super) fn parse_header(
    line: core::iter::Enumerate<core::str::Chars>,
) -> Result<Option<HeaderToken>, RequestParseError> {
    let mut key = String::new();
    let mut value = String::new();
    let mut is_key = true;

    for (_index, ch) in line {
        if ch == ':' {
            if is_key {
                is_key = false;
            } else {
                value.push(ch);
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

    use crate::parse_header;

    #[test]
    fn it_should_return_valid_headers() {
        for i in 0..10 {
            let line = format!("header{i}: value{i}");

            let result = parse_header(line.chars().enumerate())
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
        let result = parse_header("".chars().enumerate()).expect("it to be parseable");

        assert!(result.is_none());
    }
}
