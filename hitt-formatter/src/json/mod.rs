#[inline]
pub fn format_json(input: &str) -> String {
    jsonformat::format(input, jsonformat::Indentation::TwoSpace)
        .trim()
        .to_owned()
}

#[cfg(test)]
mod test_format_json {
    use crate::json::format_json;

    #[test]
    fn it_should_format_json() {
        let input = "{\"key\":\"value\"}";

        let expected_output = "{
  \"key\": \"value\"
}";

        assert_eq!(expected_output, format_json(input));
    }
}
