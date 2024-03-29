use json::format_json;

mod json;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ContentType {
    Json,
    #[default]
    Unknown,
}

impl From<&str> for ContentType {
    #[inline]
    fn from(value: &str) -> Self {
        value
            .parse::<mime::Mime>()
            .map_or(Self::Unknown, |m| match (m.type_(), m.subtype()) {
                (_, mime::JSON) => Self::Json,
                _ => Self::Unknown,
            })
    }
}

#[cfg(test)]
mod test_from_str_to_content_type {
    use crate::ContentType;

    #[test]
    fn it_should_parse_unknown_text_as_unknown() {
        for x in u8::MIN..u8::MAX {
            assert!(ContentType::Unknown == ContentType::from(x.to_string().as_str()));
            assert!(
                ContentType::Unknown == ContentType::from(format!("application/masd-{x}").as_str())
            );
            assert!(ContentType::Unknown == ContentType::from(format!("text/masd-{x}").as_str()));
        }
    }

    #[test]
    fn it_should_parse_application_json_as_json() {
        let input = "application/JSON";

        assert!(ContentType::Json == ContentType::from(input));
        assert!(ContentType::Json == ContentType::from(input.to_lowercase().as_str()));
        assert!(ContentType::Json == ContentType::from(input.to_uppercase().as_str()));

        assert!(ContentType::Json == ContentType::from("application/json; charset=utf-8"));
    }
}

#[inline]
pub fn format(input: &str, content_type: ContentType) -> Option<String> {
    match content_type {
        ContentType::Json => Some(format_json(input)),
        ContentType::Unknown => None,
    }
}

#[cfg(test)]
mod test_format {
    use crate::ContentType;

    #[test]
    fn it_should_ignore_unknown_content_types() {
        let input = "this is an unknown content type";

        assert!(crate::format(input, ContentType::Unknown).is_none());
    }

    #[test]
    fn it_should_format_json() {
        let input = "{ \"key\": \"value\" }";
        let content_type = ContentType::Json;

        assert_eq!(
            crate::format(input, content_type),
            Some(crate::json::format_json(input))
        );
    }
}
