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
    use super::parse_variable_declaration;

    #[test]
    fn it_should_parse_variable_declarations() {
        for i in 0..10 {
            let input_declaration = format!("var{i}");
            let input_value = format!("{i}");

            // NOTE: we do not start with a '@' here since it is expected to already be removed
            let input = format!("{input_declaration}={input_value}");

            match parse_variable_declaration(&mut input.chars().enumerate()) {
                Some((key, value)) => {
                    assert_eq!(input_declaration, key);
                    assert_eq!(input_value, value);
                }

                None => panic!(""),
            }
        }
    }

    #[test]
    fn it_should_trim_spaces() {
        let mut extra_spaces = String::new();

        for i in 0..10 {
            extra_spaces.push(' ');

            let input_declaration = format!("var{i}");
            let input_value = format!("{i}");

            // NOTE: we do not start with a '@' here since it is expected to already be removed
            let input = format!(
                "{input_declaration}{extra_spaces}={extra_spaces}{input_value}{extra_spaces}"
            );

            match parse_variable_declaration(&mut input.chars().enumerate()) {
                Some((key, value)) => {
                    assert_eq!(input_declaration, key);
                    assert_eq!(input_value, value);
                }

                None => panic!(""),
            }
        }
    }
}

pub enum MaybeVariable {
    Variable(String),
    Text(String),
}

#[inline]
pub fn parse_variable(chars: &mut core::iter::Enumerate<core::str::Chars>) -> MaybeVariable {
    todo!()
}

#[cfg(test)]
mod test_maybe_parse_variable {
    #[test]
    fn it_should_parse_variables() {}

    #[test]
    fn it_should_ignore_non_variables() {}
}
