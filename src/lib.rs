#![cfg_attr(test, deny(warnings))]

pub extern crate cookie;
pub extern crate futures;
pub extern crate futures_cpupool;
pub extern crate handlebars;
#[macro_use] pub extern crate hyper;
#[macro_use] extern crate log;
extern crate regex;
extern crate typemap;
pub extern crate url;
extern crate walkdir;
extern crate net2;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

mod app;
mod config;
mod error;
mod request;
mod response;
mod zircon;

pub mod extensions;
pub mod handlers;
pub mod renderers;
pub mod templates;

use futures::Future;
use futures::IntoFuture;
use std::marker::PhantomData;
use std::sync::Arc;

// Re-export hyper things
pub use hyper::server::Request as HyperRequest;
pub use hyper::server::Response as HyperResponse;

// Export zircon things
pub use app::ZirconApp;
pub use app::ZirconDefaultApp;
pub use config::Mode;
pub use config::ZirconConfig;
pub use error::ZirconError;
pub use handlers::router::Router;
pub use request::Request;
pub use response::Response;
pub use templates::HandlebarsEngine;

/// module prelude provides an easy access of zircon important modules to a user.
/// A user can import zircon importand modules etc with `use zircon::prelude::*`.
pub mod prelude {
    pub use Handler;
    pub use HandlerResult;
    pub use Request;
    pub use Response;
    pub use ZirconApp;
    pub use ZirconDefaultApp;
    pub use ZirconError;
    pub use config::ZirconConfig;
    pub use handlers::router::Router;
    pub use hyper::StatusCode;
    pub use request::Query;
    pub use templates::HandlebarsEngine;
    pub use zircon::Zircon;
}

pub type HandlerResult = Box<Future<Item=Response, Error=ZirconError>>;

pub trait Handler<A>: Send + Sync + 'static {
    fn handle(&self, Arc<A>, Request) -> HandlerResult;
}

impl<A, F> Handler<A> for F
where F: Fn(Arc<A>, Request) -> HandlerResult + Send + Sync + 'static {
    fn handle(&self, app: Arc<A>, req: Request) -> HandlerResult {
        (*self)(app, req)
    }
}

pub trait ErrorHandler<A>: Send + Sync + 'static {
    fn handle(&self, Arc<A>, &ZirconError) -> HandlerResult;
}

impl<A, F> ErrorHandler<A> for F
where F: Fn(Arc<A>, &ZirconError) -> HandlerResult + Send + Sync + 'static {
    fn handle(&self, app: Arc<A>, err: &ZirconError) -> HandlerResult {
        (*self)(app, err)
    }
}

pub struct DefaultErrorHandler<A: ZirconApp> {
    _p: PhantomData<A>
}

impl<A: ZirconApp> ErrorHandler<A> for DefaultErrorHandler<A> {
    fn handle(&self, _app: Arc<A>, err: &ZirconError) -> HandlerResult {
        let resp = error::make_default_error_response(err);
        Ok(resp).into_future().boxed()
    }
}
