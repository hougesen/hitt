#[derive(Debug)]
pub enum HittCliError {
    FailFast,
    InvalidVariableArgument(String),
    Io(std::io::Error),
    IoRead(std::path::PathBuf, std::io::Error),
    Join(tokio::task::JoinError),
    Parse(std::path::PathBuf, hitt_parser::error::RequestParseError),
    RecursiveNotEnabled,
    RequestTimeout(http::Method, http::Uri),
    Reqwest(http::Method, http::Uri, reqwest::Error),
    SSEError(Box<hitt_sse::Error>),
    SSEParseUrl(String),
    VariableArgumentKeyIndexing(String),
    VariableArgumentValueIndexing(String),
}

impl core::error::Error for HittCliError {}

impl core::fmt::Display for HittCliError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FailFast => write!(f, "exiting early since --fail-fast is enabled"),
            Self::InvalidVariableArgument(input) => write!(
                f,
                "'{input}' is not a valid variable argument - variable input should be '--var <KEY>=<VALUE>'"
            ),
            Self::Io(error) => write!(f, "io error {error:#?}"),
            Self::IoRead(path, error) => {
                write!(f, "error reading '{}' - {error:#?}", path.display())
            }
            Self::Join(error) => write!(f, "error joining handles - {error:#?}"),
            Self::Parse(path, error) => {
                write!(f, "error parsing file '{}' - {error}", path.display())
            }
            Self::RecursiveNotEnabled => {
                write!(f, "received directory path but --recursive is not enabled")
            }
            Self::RequestTimeout(method, uri) => write!(f, "{method} {uri} - request timed out"),
            Self::Reqwest(method, uri, error) => write!(f, "{method} {uri} - {error}"),
            Self::SSEError(error) => write!(f, "sse error - {error}"),
            Self::SSEParseUrl(url) => write!(f, "'{url}' is not a valid url"),
            Self::VariableArgumentKeyIndexing(variable) => {
                write!(f, "unable to index key of --var '{variable}'")
            }
            Self::VariableArgumentValueIndexing(variable) => {
                write!(f, "unable to index value of --var '{variable}'")
            }
        }
    }
}

impl From<std::io::Error> for HittCliError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
