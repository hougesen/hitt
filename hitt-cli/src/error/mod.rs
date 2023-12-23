use core::fmt;

use hitt_request::reqwest;

use crate::terminal::{TEXT_RED, TEXT_RESET};

#[derive(Debug)]
pub enum HittCliError {
    Parse(std::path::PathBuf, hitt_parser::error::RequestParseError),
    Join(tokio::task::JoinError),
    Io(std::io::Error),
    IoRead(std::path::PathBuf, std::io::Error),
    IoWrite(std::path::PathBuf, std::io::Error),
    Reqwest(
        hitt_parser::http::Method,
        hitt_parser::http::Uri,
        reqwest::Error,
    ),
}

impl fmt::Display for HittCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            HittCliError::Parse(path, error) => {
                format!("error parsing file {path:?} - {error}")
            }
            HittCliError::IoRead(path, error) => {
                format!("error reading {path:?} - {error:#?}")
            }
            HittCliError::IoWrite(path, error) => {
                format!("error writing {path:?} - {error:#?}")
            }
            HittCliError::Join(error) => format!("error joining handles - {error:#?}"),
            HittCliError::Io(error) => format!("io error {error:#?}"),
            HittCliError::Reqwest(method, uri, error) => {
                format!(
                    "error sending request {method} {uri} - {:#?}",
                    error.to_string()
                )
            }
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
