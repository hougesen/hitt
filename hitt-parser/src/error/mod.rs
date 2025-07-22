#[derive(Debug)]
pub enum RequestParseError {
    InvalidHeaderName(String),
    InvalidHeaderValue(String),
    InvalidHttpMethod(String),
    InvalidUri(String),
    MissingMethod,
    MissingUri,
    VariableNotFound(String),
}

impl core::error::Error for RequestParseError {}

impl core::fmt::Display for RequestParseError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidHeaderName(name) => write!(f, "invalid header name '{name}'"),
            Self::InvalidHeaderValue(value) => write!(f, "invalid header value '{value}'"),
            Self::InvalidHttpMethod(method) => write!(f, "invalid HTTP method '{method}'"),
            Self::InvalidUri(uri) => write!(f, "invalid uri '{uri}'"),
            Self::MissingMethod => write!(f, "missing HTTP method"),
            Self::MissingUri => write!(f, "missing uri"),
            Self::VariableNotFound(value) => write!(f, "variable '{value}' was used, but not set"),
        }
    }
}
