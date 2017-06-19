mod query;

use std::net::IpAddr;
use std::net::SocketAddr;

// TODO(mayah): Might hit https://github.com/alexcrichton/futures-rs/issues/362 ?
// use futures::Future;
use futures;
use futures::Stream;
use handlers::router::RouteResult;
use hyper::server::Request as HyperRequest;
use hyper::{self, Method, Uri, HttpVersion, Headers, Body};
use serde_json::value::Value as Json;
use typemap::TypeMap;

use prelude::*;

pub use self::query::Query;

header! { (XForwardedHost, "X-Forwarded-Host") => [String] }
header! { (XForwardedPort, "X-Forwarded-Port") => [u16] }
header! { (XForwardedProto, "X-Forwarded-Proto") => [String] }
header! { (XForwardedFor, "X-Forwarded-For") => (IpAddr)+ }

pub struct RequestHeader {
    // From HyperRequest.
    method: Method,
    uri: Uri,
    _version: HttpVersion,
    headers: Headers,
    _remote_addr: Option<SocketAddr>,

    /// Routing result.
    pub params: Option<RouteResult>,
    /// Path (if modified from the original).
    pub modified_path: Option<String>,
    /// Respect XForwarded*.
    pub respect_xforwarded: bool,
    /// Extension
    pub extensions: TypeMap,
}

impl RequestHeader {
    pub fn method(&self) -> &hyper::Method {
        &self.method
    }

    pub fn headers(&self) -> &hyper::Headers {
        &self.headers
    }

    /// Returns scheme.
    pub fn scheme(&self) -> Option<&str> {
        if self.respect_xforwarded {
            match self.headers.get::<XForwardedProto>() {
                Some(x) => Some(&x.0),
                None => self.uri.scheme(),
            }
        } else {
            self.uri.scheme()
        }
    }

    pub fn path(&self) -> &str {
        match self.modified_path {
            Some(ref x) => x,
            None => self.uri.path()
        }
    }

    pub fn modified_path(&self) -> &Option<String> {
        &self.modified_path
    }

    pub fn set_modified_path(&mut self, path: Option<String>) {
        self.modified_path = path
    }

    /// Returns routing parameter. Not from query string.
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.as_ref().unwrap().param(key)
    }

    pub fn set_params(&mut self, result: Option<RouteResult>) {
        self.params = result;
    }

    /// Parses query string.
    pub fn parse_query(&self) -> Query {
        if let Some(q) = self.uri.query() {
            Query::from_string(q)
        } else {
            Query::new()
        }
    }
}

pub struct RequestBody {
    body: Body,
}

impl RequestBody {
    /// Parses form body. multipart is not supported yet.
    ///
    /// When using this function, your source must to have `use futures::Future`.
    /// Otherwise, you will have compile error.
    pub fn parse_form_body(self) -> Box<futures::Future<Item=Query, Error=ZirconError>> {
        use futures::Future;

        let x = self.body.fold(Vec::<u8>::new(), |mut buf, chunk| {
            buf.extend_from_slice(&chunk);
            Ok::<_, hyper::Error>(buf)
        });

        x.map(|buf| {
            Query::from_string(&String::from_utf8_lossy(&buf).to_string())
        }).map_err(|err| {
            ZirconError::HyperError(err)
        }).boxed()
    }

    pub fn parse_json_body(self) -> Box<futures::Future<Item=Json, Error=ZirconError>> {
        use futures::Future;

        let x = self.body.fold(Vec::<u8>::new(), |mut buf, chunk| {
            buf.extend_from_slice(&chunk);
            Ok::<_, hyper::Error>(buf)
        });

        x.map_err(|err| {
            ZirconError::HyperError(err)
        }).and_then(|buf| {
            match ::serde_json::de::from_slice(&buf) {
                Ok(x) => Ok(x),
                Err(err) => Err(ZirconError::JsonError(err)),
            }
        }).boxed()
    }
}

pub struct Request {
    pub header: RequestHeader,
    pub body: RequestBody,
}

impl Request {
    pub fn from_internal(origin: HyperRequest) -> Request {
        let remote_addr = origin.remote_addr();
        let (method, uri, version, headers, body) = origin.deconstruct();
        Request {
            header: RequestHeader {
                method: method,
                uri: uri,
                _version: version,
                headers: headers,
                _remote_addr: remote_addr,
                params: None,
                modified_path: None,
                respect_xforwarded: false,  // TODO(mayah): Copy this from ZirconConfig.
                extensions: TypeMap::new(),
            },
            body: RequestBody {
                body: body,
            },
        }
    }

    pub fn deconstruct(self) -> (RequestHeader, RequestBody) {
        (self.header, self.body)
    }

    pub fn method(&self) -> &hyper::Method {
        self.header.method()
    }

    pub fn headers(&self) -> &hyper::Headers {
        self.header.headers()
    }

    /// Returns scheme.
    pub fn scheme(&self) -> Option<&str> {
        self.header.scheme()
    }

    pub fn path(&self) -> &str {
        self.header.path()
    }

    pub fn modified_path(&self) -> &Option<String> {
        self.header.modified_path()
    }

    pub fn set_modified_path(&mut self, path: Option<String>) {
        self.header.set_modified_path(path)
    }

    /// Returns routing parameter. Not from query string.
    pub fn param(&self, key: &str) -> Option<&str> {
        self.header.param(key)
    }

    /// Set routing parameter.
    pub fn set_params(&mut self, result: Option<RouteResult>) {
        self.header.set_params(result)
    }

    /// Parses query string.
    pub fn parse_query(&self) -> Query {
        self.header.parse_query()
    }

    pub fn extensions(&self) -> &TypeMap {
        &self.header.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.header.extensions
    }
}
