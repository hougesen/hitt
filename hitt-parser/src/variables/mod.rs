#[inline]
pub fn parse_variable_declaration(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
) -> Option<(String, String)> {
    let mut declaration = String::new();

    let mut value = String::new();

    let mut is_declaration = true;

    for (_, ch) in chars {
        if ch == '=' && is_declaration {
            is_declaration = false;
        } else if is_declaration {
            declaration.push(ch);
        } else {
            value.push(ch);
        }
    }

    if is_declaration {
        return None;
    }

    Some((declaration.trim().to_owned(), value.trim().to_owned()))
}

#[cfg(test)]
mod test_parse_variable_declarations {
    use crate::to_enum_chars;

    use super::parse_variable_declaration;

    #[test]
    fn it_should_parse_variable_declarations() {
        for i in i8::MIN..i8::MAX {
            let input_declaration = format!("var{i}");
            let input_value = format!("{i}");

            // NOTE: we do not start with a '@' here since it is expected to already be removed
            let input = format!("{input_declaration}={input_value}");

            match parse_variable_declaration(&mut to_enum_chars(&input)) {
                Some((key, value)) => {
                    assert_eq!(input_declaration, key);
                    assert_eq!(input_value, value);
                }

                None => panic!("it should return a valid string"),
            }
        }
    }

    #[test]
    fn it_should_allow_emails() {
        let mut extra_spaces = String::new();

        for i in i8::MIN..i8::MAX {
            extra_spaces.push(' ');

            let input_declaration = format!("var{i}");
            let input_value = format!("mads{i}@mhouge.dk");

            // NOTE: we do not start with a '@' here since it is expected to already be removed
            let input = format!(
                "{input_declaration}{extra_spaces}={extra_spaces}{input_value}{extra_spaces}"
            );

            match parse_variable_declaration(&mut to_enum_chars(&input)) {
                Some((key, value)) => {
                    assert_eq!(input_declaration, key);
                    assert_eq!(input_value, value);
                }

                None => panic!("it should return a variable declaration"),
            }
        }
    }
}

#[inline]
pub fn parse_variable(
    chars: &mut core::iter::Enumerate<core::str::Chars>,
) -> Option<(String, usize)> {
    let mut jumps = 0;

    if let Some((_, '{')) = chars.next() {
        jumps += 1;
        let mut x = String::new();

        let mut is_key = true;

        while let Some((_, ch)) = chars.next() {
            jumps += 1;

            if ch == '{' {
                return None;
            }

            if ch == '}' {
                if let Some((_, '}')) = chars.next() {
                    if x.is_empty() {
                        return None;
                    }

                    jumps += 1;
                    return Some((x, jumps));
                }

                return None;
            }

            if ch.is_whitespace() {
                if !x.is_empty() {
                    is_key = false;
                }
            } else if !is_key {
                return None;
            } else {
                x.push(ch);
            }
        }
    };

    None
}

#[cfg(test)]
mod test_maybe_parse_variable {
    use crate::to_enum_chars;

    use super::parse_variable;

    #[test]
    fn it_should_parse_variables() {
        let before = "{";
        let after = "}}";

        for i in i8::MIN..i8::MAX {
            let input_name = format!("name{i}");

            // NOTE: the first '{' was consumed by the caller
            let input = format!("{before}{input_name}{after}");

            match parse_variable(&mut to_enum_chars(&input)) {
                Some((output_name, jumps)) => {
                    assert_eq!(input_name, output_name);
                    assert_eq!(input.len(), jumps);
                }
                None => panic!("it should not return none"),
            }
        }
    }

    #[test]
    fn should_trim_variable_whitespace() {
        let mut extra_whitespace = String::new();

        let before = "{";
        let after = "}}";

        for i in i8::MIN..i8::MAX {
            extra_whitespace.push(' ');

            let input_name = format!("name{i}");

            // NOTE: the first '{' was consumed by the caller
            let input = format!("{before}{extra_whitespace}{input_name}{extra_whitespace}{after}");

            match parse_variable(&mut to_enum_chars(&input)) {
                Some((output_name, jumps)) => {
                    assert_eq!(input_name, output_name);
                    assert_eq!(input.len(), jumps);
                }
                None => panic!("it should not return text"),
            }
        }
    }

    #[test]
    fn it_should_ignore_non_variables() {
        let inputs = [
            " name ",
            " { name n }} ",
            " { name } }",
            " { name",
            " { name} }",
            " { name}",
            " { name}{",
            "name   }}  ",
            "name }}  ",
            "name",
            "name} ",
            "{ name",
            "{ name} }",
            "{ {name} }",
            "{name n}}",
            "{name",
            "{name} }",
            "{name}",
            "{{name x}",
            "{{name}",
            "{{name}}",
            "{} name }}",
        ];

        for input in inputs {
            if let Some((var, jumps)) = parse_variable(&mut to_enum_chars(input)) {
                panic!("expected None but received '{var}' {jumps}' from '{input}''");
            }
        }
    }
}