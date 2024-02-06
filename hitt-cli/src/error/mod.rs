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
}

impl core::fmt::Display for HittCliError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let error_message = match self {
            Self::Parse(path, error) => format!("error parsing file {path:?} - {error}"),
            Self::IoRead(path, error) => format!("error reading {path:?} - {error:#?}"),
            Self::Join(error) => format!("error joining handles - {error:#?}"),
            Self::Io(error) => format!("io error {error:#?}"),
            Self::Reqwest(method, uri, error) => format!("{method} {uri} - {error}"),
            Self::RequestTimeout(method, uri) => format!("{method} {uri} - request timed out"),
            Self::InvalidVariableArgument(input) => {
                format!("'{input}' is not a valid variable argument - variable input should be '--var <KEY>=<VALUE>'")
            }
            Self::VariableArgumentKeyIndexing(variable) => {
                format!("unable to index key of --var '{variable}'")
            }
            Self::VariableArgumentValueIndexing(variable) => {
                format!("unable to index value of --var '{variable}'")
            }
        };

        write!(f, "hitt: {error_message}")
    }
}

impl From<tokio::task::JoinError> for HittCliError {
    #[inline]
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Join(value)
    }
}

impl From<std::io::Error> for HittCliError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
