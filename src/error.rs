use reqwest::Error as HttpError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::{fmt, fmt::Display};

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    Http(HttpError),
    Custom(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Error::Json(json_e) => format!("Json Error: {:?}", json_e),
            Error::Http(http_e) => format!("Http Error: {:?}", http_e),
            Error::Custom(custom_e) => format!("Error: {:?}", custom_e),
        };
        f.write_str(&reason)
    }
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Error {
        Error::Http(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Error {
        Error::Json(e)
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
