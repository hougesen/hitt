use crate::variables::parse_variable;

#[inline]
pub fn parse_http_version(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
    vars: &std::collections::HashMap<String, String>,
) -> Option<http::version::Version> {
    let mut version = String::new();

    while let Some((_, ch)) = chars.next() {
        if ch.is_whitespace() {
            if !version.is_empty() {
                break;
            }

            continue;
        } else if ch == '{' {
            // FIXME: remove clone
            if let Some((var, jumps)) = parse_variable(&mut chars.clone()) {
                if let Some(var_value) = vars.get(&var) {
                    version.push_str(var_value);

                    for _ in 0..jumps {
                        chars.next();
                    }

                    continue;
                }

                // NOTE: should variable not existing raise an error?
            }
        }

        version.push(ch);
    }

    if version.is_empty() {
        return None;
    }

    match version.to_lowercase().trim() {
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
    use crate::{to_enum_chars, version::parse_http_version};

    static EMPTY_VARS: std::sync::LazyLock<std::collections::HashMap<String, String>> =
        std::sync::LazyLock::new(std::collections::HashMap::new);

    const HTTP_0_9_INPUTS: [&str; 5] = [
        "http/0.9",
        "HTTP/0.9",
        "   HTTP/0.9",
        "HTTP/0.9   ",
        "   HTTP/0.9   ",
    ];

    #[test]
    fn it_should_parse_http_0_9() {
        for input in HTTP_0_9_INPUTS {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_09, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_09, lowercase_result);
        }
    }

    const HTTP_1_0_INPUTS: [&str; 10] = [
        "http/1",
        "http/1.0",
        "HTTP/1",
        "   HTTP/1",
        "HTTP/1   ",
        "   HTTP/1   ",
        "HTTP/1.0",
        "   HTTP/1.0",
        "HTTP/1.0   ",
        "   HTTP/1.0   ",
    ];

    #[test]
    fn it_should_parse_http_1_0() {
        for input in HTTP_1_0_INPUTS {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_10, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_10, lowercase_result);
        }
    }

    // NOTE: should HTTP/1 mean the same as HTTP/1.1?
    const HTTP_1_1_INPUTS: [&str; 5] = [
        "http/1.1",
        "HTTP/1.1",
        "   HTTP/1.1",
        "HTTP/1.1   ",
        "   HTTP/1.1   ",
    ];

    #[test]
    fn it_should_parse_http_1_1() {
        for input in HTTP_1_1_INPUTS {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_11, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_11, lowercase_result);
        }
    }

    const HTTP_2_0_INPUTS: [&str; 10] = [
        "http/2",
        "http/2.0",
        "HTTP/2",
        "   HTTP/2",
        "HTTP/2   ",
        "   HTTP/2   ",
        "HTTP/2.0",
        "   HTTP/2.0",
        "HTTP/2.0   ",
        "   HTTP/2.0   ",
    ];

    #[test]
    fn it_should_parse_http_2_0() {
        for input in HTTP_2_0_INPUTS {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_2, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_2, lowercase_result);
        }
    }

    const HTTP_3_0_INPUTS: [&str; 10] = [
        "http/3.0",
        "http/3",
        "HTTP/3",
        "   HTTP/3",
        "HTTP/3   ",
        "   HTTP/3   ",
        "HTTP/3.0",
        "   HTTP/3.0",
        "HTTP/3.0   ",
        "   HTTP/3.0   ",
    ];

    #[test]
    fn it_should_parse_http_3_0() {
        for input in HTTP_3_0_INPUTS {
            let uppercase_result = parse_http_version(&mut to_enum_chars(input), &EMPTY_VARS)
                .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_3, uppercase_result);

            let lowercase_result =
                parse_http_version(&mut to_enum_chars(&input.to_lowercase()), &EMPTY_VARS)
                    .expect("it to return a http version");

            assert_eq!(http::Version::HTTP_3, lowercase_result);
        }
    }

    #[test]
    fn it_should_support_variables() {
        let mut vars = std::collections::HashMap::new();

        for version in HTTP_0_9_INPUTS {
            vars.insert("version".to_owned(), version.to_owned());

            assert_eq!(
                Some(http::Version::HTTP_09),
                parse_http_version(&mut to_enum_chars("{{version}}"), &vars)
            );

            assert_eq!(
                Some(http::Version::HTTP_09),
                parse_http_version(&mut to_enum_chars("{{  version  }}"), &vars)
            );
        }

        for version in HTTP_1_0_INPUTS {
            vars.insert("version".to_owned(), version.to_owned());

            assert_eq!(
                Some(http::Version::HTTP_10),
                parse_http_version(&mut to_enum_chars("{{version}}"), &vars)
            );

            assert_eq!(
                Some(http::Version::HTTP_10),
                parse_http_version(&mut to_enum_chars("{{  version  }}"), &vars)
            );
        }

        for version in HTTP_1_1_INPUTS {
            vars.insert("version".to_owned(), version.to_owned());

            assert_eq!(
                Some(http::Version::HTTP_11),
                parse_http_version(&mut to_enum_chars("{{version}}"), &vars)
            );

            assert_eq!(
                Some(http::Version::HTTP_11),
                parse_http_version(&mut to_enum_chars("{{  version  }}"), &vars)
            );
        }

        for version in HTTP_2_0_INPUTS {
            vars.insert("version".to_owned(), version.to_owned());

            assert_eq!(
                Some(http::Version::HTTP_2),
                parse_http_version(&mut to_enum_chars("{{version}}"), &vars)
            );

            assert_eq!(
                Some(http::Version::HTTP_2),
                parse_http_version(&mut to_enum_chars("{{  version  }}"), &vars)
            );
        }

        for version in HTTP_3_0_INPUTS {
            vars.insert("version".to_owned(), version.to_owned());

            assert_eq!(
                Some(http::Version::HTTP_3),
                parse_http_version(&mut to_enum_chars("{{version}}"), &vars)
            );

            assert_eq!(
                Some(http::Version::HTTP_3),
                parse_http_version(&mut to_enum_chars("{{  version  }}"), &vars)
            );
        }
    }

    #[test]
    fn it_should_ignore_unknown_http_versions() {
        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("unknown"), &EMPTY_VARS)
        );

        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("{unknown"), &EMPTY_VARS)
        );

        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("{{unknown"), &EMPTY_VARS)
        );

        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("{{unknown}"), &EMPTY_VARS)
        );

        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("{{unknown} }"), &EMPTY_VARS)
        );
    }

    #[test]
    fn it_should_return_none_if_var_isnt_found() {
        // NOTE: should it raise an error instead?

        assert_eq!(
            None,
            parse_http_version(&mut to_enum_chars("{{unknown}}"), &EMPTY_VARS)
        );
    }
}
