use std;
use std::error::Error;

use futures::{Future, IntoFuture};
use hyper;
use prelude::*;
use serde_json;

pub enum ZirconError {
    /// An error that specifies only http status.
    Status(StatusCode),
    /// An error that specifies http status and error message.
    StringError(StatusCode, String),
    IoError(std::io::Error),
    HyperError(hyper::Error),
    JsonError(serde_json::Error),
}

impl ZirconError {
    pub fn render(self) -> HandlerResult {
        Err(self).into_future().boxed()
    }

    pub fn message<T: Into<String>>(code: StatusCode, message: T) -> ZirconError {
        ZirconError::StringError(code, message.into())
    }

    pub fn render_error_status(code: StatusCode) -> HandlerResult {
        Err(ZirconError::Status(code)).into_future().boxed()
    }

    pub fn render_error_message<T: Into<String>>(code: StatusCode, message: T) -> HandlerResult {
        Err(ZirconError::StringError(code, message.into())).into_future().boxed()
    }
}

impl From<String> for ZirconError {
    fn from(err: String) -> ZirconError {
        ZirconError::StringError(StatusCode::InternalServerError, err)
    }
}

impl<'a> From<&'a str> for ZirconError {
    fn from(err: &'a str) -> ZirconError {
        ZirconError::StringError(StatusCode::InternalServerError, err.to_string())
    }
}

impl From<std::io::Error> for ZirconError {
    fn from(err: std::io::Error) -> ZirconError {
        ZirconError::IoError(err)
    }
}

pub fn make_default_error_response(err: &ZirconError) -> Response {
    let mut resp = Response::new();
    match err {
        &ZirconError::Status(code) => {
            resp.origin.set_status(code);
        },
        &ZirconError::StringError(code, ref message) => {
            resp.origin.set_status(code);
            resp.origin.set_body(message.to_string());
        },
        &ZirconError::IoError(ref io_err) => {
            resp.origin.set_status(StatusCode::InternalServerError);
            resp.origin.set_body(io_err.description().to_string());
        },
        &ZirconError::HyperError(ref hyper_err) => {
            resp.origin.set_status(StatusCode::InternalServerError);
            resp.origin.set_body(hyper_err.description().to_string());
        },
        &ZirconError::JsonError(ref json_err) => {
            resp.origin.set_status(StatusCode::InternalServerError);
            resp.origin.set_body(json_err.description().to_string());
        }
    }

    resp
}

pub fn make_fallback_error_response() -> Response {
    let mut resp = Response::new();
    resp.origin.set_status(StatusCode::InternalServerError);
    resp
}
