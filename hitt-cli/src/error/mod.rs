use core::fmt;

use crate::terminal::{TEXT_RED, TEXT_RESET};

#[derive(Debug)]
pub enum HittCliError {
    Parse(std::path::PathBuf, hitt_parser::error::RequestParseError),
    Join(tokio::task::JoinError),
    Io(std::io::Error),
    IoRead(std::path::PathBuf, std::io::Error),
    IoWrite(std::path::PathBuf, std::io::Error),
    Reqwest(http::Method, http::Uri, reqwest::Error),
    RequestTimeout(http::Method, http::Uri),
}

impl fmt::Display for HittCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            Self::Parse(path, error) => format!("error parsing file {path:?} - {error}"),
            Self::IoRead(path, error) => format!("error reading {path:?} - {error:#?}"),
            Self::IoWrite(path, error) => format!("error writing {path:?} - {error:#?}"),
            Self::Join(error) => format!("error joining handles - {error:#?}"),
            Self::Io(error) => format!("io error {error:#?}"),
            Self::Reqwest(method, uri, error) => format!("{method} {uri} - {error}"),
            Self::RequestTimeout(method, uri) => format!("{method} {uri} - request timed out"),
        };

        write!(f, "{TEXT_RED}hitt: {error_message}{TEXT_RESET}")
    }
}

impl From<tokio::task::JoinError> for HittCliError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Join(value)
    }
}

impl From<std::io::Error> for HittCliError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
