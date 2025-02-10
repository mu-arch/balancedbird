use std::error::Error;
use scraper::error::SelectorErrorKind;


#[derive(Debug)]
pub enum AppErrorInternal {
    AddrParseError(std::net::AddrParseError),
    //AcmeError(acme_micro::Error),
    Custom(String),
}

impl From<std::net::AddrParseError> for AppErrorInternal {
    fn from(inner: std::net::AddrParseError) -> Self {
        AppErrorInternal::AddrParseError(inner)
    }
}


/*
impl From<acme_micro::Error> for AppErrorInternal {
    fn from(inner: acme_micro::Error) -> Self {
        AppErrorInternal::AcmeError(inner)
    }
}

 */


use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppErrorExternal {
    IoError(std::io::Error),
    AxumError(axum::http::Error),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    SelectorError(Box<dyn Error + Send + Sync>),
    FileNotFound,
    PathTraversal,
    Custom(String)
}

impl IntoResponse for AppErrorExternal {
    fn into_response(self) -> Response {
        dbg!(&self);

        let (status, error_message) = match self {
            AppErrorExternal::IoError(ref e) if e.kind() == std::io::ErrorKind::NotFound =>
                (StatusCode::NOT_FOUND, "File not found"),
            AppErrorExternal::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal IO Error"),
            AppErrorExternal::FileNotFound => (StatusCode::NOT_FOUND, "File not found"),
            AppErrorExternal::PathTraversal => (StatusCode::FORBIDDEN, "nice try"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
        };

        (status, error_message).into_response()
    }
}

impl From<std::io::Error> for AppErrorExternal {
    fn from(inner: std::io::Error) -> Self {
        AppErrorExternal::IoError(inner)
    }
}

impl From<std::num::ParseIntError> for AppErrorExternal {
    fn from(err: std::num::ParseIntError) -> Self {
        AppErrorExternal::ParseIntError(err)
    }
}

impl From<SelectorErrorKind<'_>> for AppErrorExternal {
    fn from(err: SelectorErrorKind<'_>) -> Self {
        AppErrorExternal::SelectorError(err.to_string().into()) // Convert error to String
    }
}

impl From<std::num::ParseFloatError> for AppErrorExternal {
    fn from(err: std::num::ParseFloatError) -> Self {
        AppErrorExternal::ParseFloatError(err)
    }
}


impl From<axum::http::Error> for AppErrorExternal {
    fn from(inner: axum::http::Error) -> Self {
        AppErrorExternal::AxumError(inner)
    }
}