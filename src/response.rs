use futures::Future;
use futures::IntoFuture;
use hyper::header::{ContentLength, ContentType, Header};
use hyper::server::Response as HyperResponse;

use prelude::*;

/// Response represents http response.
pub struct Response {
    /// Hyper original response.
    pub origin: HyperResponse,
}

impl Response {
    /// Creates a fresh response.
    pub fn new() -> Response {
        Response {
            origin: HyperResponse::new(),
        }
    }

    /// Converts Response to HandlerResult.
    /// You need to call render() after building Response.
    pub fn render(self) -> HandlerResult {
        Ok(self).into_future().boxed()
    }

    /// Creates a Response that contains only text.
    pub fn text<T: Into<String>>(text: T) -> Response {
        let mut resp = Response::new();
        let s = text.into();
        resp.origin.headers_mut().set(ContentLength(s.len() as u64));
        resp.origin.headers_mut().set(ContentType::plaintext());
        resp.origin.set_body(s);
        resp
    }

    /// Creates a json Response.
    pub fn json<T: Into<String>>(obj: T) -> Response {
        let mut resp = Response::new();
        let s = obj.into();
        resp.origin.headers_mut().set(ContentLength(s.len() as u64));
        resp.origin.headers_mut().set(ContentType::json());
        resp.origin.set_body(s);
        resp
    }

    /// Creates a Response for redirect.
    pub fn redirect(url: &str) -> Response {
        let mut resp = Response::new();
        resp.origin.set_status(StatusCode::Found);
        resp.origin.headers_mut().set_raw("location", vec![url.as_bytes().to_vec()]);
        resp
    }

    /// Set header in Response.
    /// Use like a builder pattern.
    pub fn with_header<H: Header>(mut self, h: H) -> Response {
        self.origin.headers_mut().set(h);
        self
    }

    /// Set status code in Response.
    /// Use like a builder pattern.
    pub fn with_status(mut self, status: StatusCode) -> Response {
        self.origin.set_status(status);
        self
    }
}
