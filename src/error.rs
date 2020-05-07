//! The error type for this library.

use http::header::ToStrError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::{fmt, fmt::Display};

/// The generic error type used for handling errors within this library.
#[derive(Debug)]
pub enum Error {
    /// Any other error which couldn't be represented well otherwise.
    Custom(String),
    /// A header is missing.
    MissingHeader(String),
    /// I wish there was a better way to do this.
    ParseFloatError(ParseFloatError),
    /// I wish there was a better way to do this.
    ParseIntError(ParseIntError),
    /// Reqwest errors.
    Reqwest(ReqwestError),
    /// Serde errors.
    Serde(SerdeError),
    /// I wish there was a better way to do this.
    ToStrError(ToStrError),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Error::Custom(custom_e) => format!("Error: {:?}", custom_e),
            Error::MissingHeader(header_e) => format!("Missing Header: {:?}", header_e),
            Error::ParseFloatError(parsefloat_e) => format!("ParseFloat Error: {:?}", parsefloat_e),
            Error::ParseIntError(parseint_e) => format!("ParseInt Error: {:?}", parseint_e),
            Error::Reqwest(http_e) => format!("Reqwest Error: {:?}", http_e),
            Error::Serde(json_e) => format!("Serde Error: {:?}", json_e),
            Error::ToStrError(tostr_e) => format!("ToStr Error: {:?}", tostr_e),
        };
        f.write_str(&reason)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseIntError(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Error {
        Error::ParseFloatError(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error {
        Error::Reqwest(e)
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Error {
        Error::Serde(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::Custom(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::Custom(e.to_string())
    }
}

impl From<ToStrError> for Error {
    fn from(e: ToStrError) -> Error {
        Error::ToStrError(e)
    }
}
