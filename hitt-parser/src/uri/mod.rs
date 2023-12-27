use core::str::FromStr;

use crate::{error::RequestParseError, RequestToken};

impl From<http::uri::Uri> for RequestToken {
    #[inline]
    fn from(value: http::uri::Uri) -> Self {
        Self::Uri(value)
    }
}

#[inline]
pub(super) fn parse_uri_input(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
) -> Result<http::uri::Uri, RequestParseError> {
    let mut uri = String::new();

    for (_i, ch) in chars {
        if ch.is_whitespace() {
            if !uri.is_empty() {
                break;
            }
        } else {
            uri.push(ch);
        }
    }

    http::uri::Uri::from_str(&uri).map_err(|_err| RequestParseError::InvalidUri(uri))
}

#[cfg(test)]
mod test_parse_uri_input {
    use crate::uri::parse_uri_input;

    #[test]
    fn it_should_be_able_to_parse_uris() {
        let input_uris = [
            "https://mhouge.dk/",
            "https://goout.dk/",
            "https://mhouge.dk?key=value",
        ];

        for input_uri in input_uris {
            let result = parse_uri_input(&mut format!("{input_uri} HTTP/2").chars().enumerate());

            assert!(result.is_ok());
        }
    }

    #[test]
    fn it_should_ignore_leading_spaces() {
        let input_uri = "https://mhouge.dk/";

        let result =
            parse_uri_input(&mut format!("         {input_uri} HTTP/2.0").chars().enumerate())
                .expect("it should return a valid uri");

        assert_eq!(result.to_string(), input_uri)
    }

    #[test]
    fn it_should_reject_invalid_uris() {
        let invalid_uris = ["m:a:d:s"];

        for invalid_uri in invalid_uris {
            parse_uri_input(&mut format!("{invalid_uri} HTTP/2").chars().enumerate())
                .expect_err("it should return an error");
        }
    }

    #[test]
    fn it_should_support_query_paramers() {
        let input_uri = "https://mhouge.dk/";

        for i in 0..10 {
            let result =
                parse_uri_input(&mut format!("{input_uri}?key{i}=value{i}").chars().enumerate())
                    .expect("it should return a valid uri");

            assert_eq!(result.to_string(), format!("{input_uri}?key{i}=value{i}"));
        }
    }
}
