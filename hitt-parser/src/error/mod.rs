#[derive(Debug)]
pub enum RequestParseError {
    InvalidHttpMethod(String),
    InvalidUri(String),
    MissingMethod,
    MissingUri,
    InvalidHeaderName(String),
    InvalidHeaderValue(String),
}

impl core::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidHttpMethod(method) => write!(f, "invalid HTTP method '{method}'"),
            Self::InvalidUri(uri) => write!(f, "invalid uri '{uri}'"),
            Self::MissingMethod => write!(f, "missing HTTP method"),
            Self::MissingUri => write!(f, "missing uri"),
            Self::InvalidHeaderName(name) => write!(f, "invalid header name '{name}'"),
            Self::InvalidHeaderValue(value) => write!(f, "invalid header value '{value}'"),
        }
    }
}
