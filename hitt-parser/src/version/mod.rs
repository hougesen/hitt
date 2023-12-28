#[inline]
pub fn parse_http_version(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
    _vars: &std::collections::HashMap<String, String>,
) -> Option<http::version::Version> {
    let mut version = String::new();

    for (_, ch) in chars {
        if ch.is_whitespace() {
            if !version.is_empty() {
                break;
            }

            continue;
        }

        version.push(ch);
    }

    if version.is_empty() {
        return None;
    }

    match version.to_lowercase().as_str() {
        "http/0.9" => Some(http::Version::HTTP_09),
        "http/1.0" | "http/1" => Some(http::Version::HTTP_10),
        "http/1.1" => Some(http::Version::HTTP_11),
        "http/2.0" | "http/2" => Some(http::Version::HTTP_2),
        "http/3.0" | "http/3" => Some(http::Version::HTTP_3),
        _ => None,
    }
}

#[cfg(test)]
mod test_parse_http_version {
    use once_cell::sync::Lazy;

    use crate::{to_enum_chars, version::parse_http_version};

    static EMPTY_VARS: Lazy<std::collections::HashMap<String, String>> =
        Lazy::new(std::collections::HashMap::new);

    #[test]
    fn it_should_parse_http_0_9() {
        let inputs = ["HTTP/0.9", "   HTTP/0.9", "HTTP/0.9   ", "   HTTP/0.9   "];

        for input in inputs {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_09, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_09, lowercase_result);
        }
    }

    #[test]
    fn it_should_parse_http_1_0() {
        let inputs = [
            "HTTP/1",
            "   HTTP/1",
            "HTTP/1   ",
            "   HTTP/1   ",
            "HTTP/1.0",
            "   HTTP/1.0",
            "HTTP/1.0   ",
            "   HTTP/1.0   ",
        ];

        for input in inputs {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_10, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_10, lowercase_result);
        }
    }

    #[test]
    fn it_should_parse_http_1_1() {
        // NOTE: should HTTP/1 mean the same as HTTP/1.1?
        let inputs = ["HTTP/1.1", "   HTTP/1.1", "HTTP/1.1   ", "   HTTP/1.1   "];

        for input in inputs {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_11, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_11, lowercase_result);
        }
    }

    #[test]
    fn it_should_parse_http_2_0() {
        let inputs = [
            "HTTP/2",
            "   HTTP/2",
            "HTTP/2   ",
            "   HTTP/2   ",
            "HTTP/2.0",
            "   HTTP/2.0",
            "HTTP/2.0   ",
            "   HTTP/2.0   ",
        ];

        for input in inputs {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_2, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_2, lowercase_result);
        }
    }

    #[test]
    fn it_should_parse_http_3_0() {
        let inputs = [
            "HTTP/3",
            "   HTTP/3",
            "HTTP/3   ",
            "   HTTP/3   ",
            "HTTP/3.0",
            "   HTTP/3.0",
            "HTTP/3.0   ",
            "   HTTP/3.0   ",
        ];

        for input in inputs {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_3, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_3, lowercase_result);
        }
    }
}
