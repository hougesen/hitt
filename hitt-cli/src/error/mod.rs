#[derive(Debug)]
pub enum HittCliError {
    Parse(std::path::PathBuf, hitt_parser::error::RequestParseError),
    Join(tokio::task::JoinError),
    Io(std::io::Error),
    IoRead(std::path::PathBuf, std::io::Error),
    Reqwest(http::Method, http::Uri, reqwest::Error),
    RequestTimeout(http::Method, http::Uri),
    InvalidVariableArgument(String),
    VariableArgumentKeyIndexing(String),
    VariableArgumentValueIndexing(String),
    RecursiveNotEnabled,
    FailFast,
    SSEParseUrl(String),
    SSEError(hitt_sse::Error),
}

impl core::error::Error for HittCliError {}

impl core::fmt::Display for HittCliError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Parse(path, error) => write!(f, "error parsing file {path:?} - {error}"),
            Self::IoRead(path, error) => write!(f, "error reading {path:?} - {error:#?}"),
            Self::Join(error) => write!(f, "error joining handles - {error:#?}"),
            Self::Io(error) => write!(f, "io error {error:#?}"),
            Self::Reqwest(method, uri, error) => write!(f, "{method} {uri} - {error}"),
            Self::RequestTimeout(method, uri) => write!(f, "{method} {uri} - request timed out"),
            Self::InvalidVariableArgument(input) => {
                write!(
                    f,
                    "'{input}' is not a valid variable argument - variable input should be '--var <KEY>=<VALUE>'"
                )
            }
            Self::VariableArgumentKeyIndexing(variable) => {
                write!(f, "unable to index key of --var '{variable}'")
            }
            Self::VariableArgumentValueIndexing(variable) => {
                write!(f, "unable to index value of --var '{variable}'")
            }
            Self::RecursiveNotEnabled => {
                write!(f, "received directory path but --recursive is not enabled")
            }
            Self::FailFast => write!(f, "exiting early since --fail-fast is enabled"),
            Self::SSEParseUrl(url) => write!(f, "'{url}' is not a valid url"),
            Self::SSEError(error) => write!(f, "sse error - {error}"),
        }
    }
}

impl From<std::io::Error> for HittCliError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
