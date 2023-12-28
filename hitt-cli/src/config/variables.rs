use crate::error::HittCliError;

pub fn parse_variable_argument(argument: &str) -> Result<(String, String), HittCliError> {
    let pos = argument
        .find('=')
        .ok_or_else(|| HittCliError::InvalidVariableArgument(argument.to_owned()))?;

    let key = argument.get(..pos).unwrap().to_owned();

    let val = argument.get(pos + 1..).unwrap().to_owned();

    Ok((key, val))
}

#[cfg(test)]
mod test_parse_variable_argument {
    use crate::config::variables::parse_variable_argument;

    #[test]
    fn it_should_parse_valid_arguments() {
        for i in u8::MIN..u8::MAX {
            let key = format!("key{i}");
            let value = format!("value{i}");

            let input = format!("{key}={value}");

            let result = parse_variable_argument(&input).expect("it to return a variable");

            assert_eq!(result.0, key);
            assert_eq!(result.1, value);
        }
    }

    #[test]
    fn it_should_reject_if_no_equal_sign() {
        for i in u8::MIN..u8::MAX {
            let key = format!("key{i}");
            let value = format!("value{i}");

            let input = format!("{key}{value}");

            parse_variable_argument(&input).expect_err("it to return a variable");
        }
    }
}
