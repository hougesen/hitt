#[derive(Debug)]
pub enum RequestParseError {
    InvalidHttpMethod(String),
    InvalidUri(String),
    MissingMethod,
    MissingUri,
    InvalidHeaderName(String),
    InvalidHeaderValue(String),
}

impl std::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestParseError::InvalidHttpMethod(method) => {
                write!(f, "invalid HTTP method '{method}'")
            }
            RequestParseError::InvalidUri(uri) => write!(f, "invalid uri '{uri}'"),
            RequestParseError::MissingMethod => write!(f, "missing HTTP method"),
            RequestParseError::MissingUri => write!(f, "missing uri"),
            RequestParseError::InvalidHeaderName(name) => write!(f, "invalid header name '{name}'"),
            RequestParseError::InvalidHeaderValue(value) => {
                write!(f, "invalid header value '{value}'")
            }
        }
    }
}