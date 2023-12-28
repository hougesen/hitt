#[inline]
pub fn format_json(input: &str) -> String {
    jsonformat::format(input, jsonformat::Indentation::TwoSpace)
}
