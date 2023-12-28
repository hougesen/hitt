use json::format_json;

mod json;

#[derive(Copy, Clone)]
pub enum ContentType {
    Json,
    Unknown,
}

impl From<&str> for ContentType {
    #[inline]
    fn from(value: &str) -> Self {
        if value.starts_with("application/json") {
            return Self::Json;
        }

        Self::Unknown
    }
}

#[inline]
pub fn format(input: &str, content_type: ContentType) -> Option<String> {
    match content_type {
        ContentType::Json => Some(format_json(input)),
        ContentType::Unknown => None,
    }
}
