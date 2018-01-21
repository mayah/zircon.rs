use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;

use futures::Stream;
use futures::{Future, IntoFuture};
use hyper::server::Http;
use hyper;
use net2::unix::UnixTcpBuilderExt;
use net2;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use DefaultErrorHandler;
use ErrorHandler;
use HyperRequest;
use HyperResponse;
use error;
use prelude::*;

struct ZirconService<A: ZirconApp, H: Handler<A>, E: ErrorHandler<A>> {
    app: Arc<A>,
    handler: Arc<H>,
    error_handler: Arc<E>,
}

impl<A: ZirconApp, H: Handler<A>, E: ErrorHandler<A>> hyper::server::Service for ZirconService<A, H, E> {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, hyper_request: HyperRequest) -> Self::Future {
        // Need to clone self.data because of lifetime.
        // If this works without clone, it is good.

        let req = Request::from_internal(hyper_request);

        let x1 = self.handler.handle(self.app.clone(), req).map(move |resp| {
            resp.origin
        });

        // Need move closure to take ownership of a2 and e2.
        let a2 = self.app.clone();
        let e2 = self.error_handler.clone();
        let x2 = x1.or_else(move |err| {
            e2.handle(a2, &err).map(|resp| {
                resp.origin
            }).or_else(|_err2| {
                Ok(error::make_fallback_error_response().origin)
            })
        });

        // Hmm, Box::new() works but .boxed() doesn't work.
        // See https://github.com/alexcrichton/futures-rs/issues/363
        Box::new(x2.into_future())
    }
}

// ----------------------------------------------------------------------

pub struct Zircon<A: ZirconApp, H: Handler<A>, E: ErrorHandler<A>> {
    app: Arc<A>,
    handler: Arc<H>,
    error_handler: Arc<E>,
}

impl<A: ZirconApp, H: Handler<A>> Zircon<A, H, DefaultErrorHandler<A>> {
    pub fn new(app: A, handler: H) -> Zircon<A, H, DefaultErrorHandler<A>> {
        Zircon::with_custom_error_handler(app, handler, DefaultErrorHandler {
            _p: PhantomData,
        })
    }

    pub fn app(&self) -> Arc<A> {
        self.app.clone()
    }
}

fn serve<A: ZirconApp, H: Handler<A>, E: ErrorHandler<A>>(addr: SocketAddr, protocol: Arc<Http>,
                                                          app: Arc<A>, handler: Arc<H>, error_handler: Arc<E>) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = net2::TcpBuilder::new_v4().unwrap()
        .reuse_port(true).unwrap()
        .bind(&addr).unwrap()
        .listen(128).unwrap();
    let listener = TcpListener::from_listener(listener, &addr, &handle).unwrap();

    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, ZirconService {
            app: app.clone(),
            handler: handler.clone(),
            error_handler: error_handler.clone(),
        });
        Ok(())
    })).unwrap();
}

impl<A: ZirconApp, H: Handler<A>, E: ErrorHandler<A>> Zircon<A, H, E> {
    pub fn with_custom_error_handler(app: A, handler: H, error_handler: E) -> Zircon<A, H, E> {
        Zircon {
            app: Arc::new(app),
            handler: Arc::new(handler),
            error_handler: Arc::new(error_handler),
        }
    }

    pub fn http(self, addr: &SocketAddr) -> hyper::Result<()> {
        // Example taken from https://gist.github.com/alexcrichton/7b97beda66d5e9b10321207cd69afbbc
        let protocol = Arc::new(Http::new());

        for _ in 0..self.app.num_accept_threads() {
            let addr = addr.clone();
            let app = self.app.clone();
            let handler = self.handler.clone();
            let error_handler = self.error_handler.clone();
            let p = protocol.clone();
            thread::spawn(move || serve(addr.clone(), p.clone(), app.clone(), handler.clone(), error_handler.clone()));
        }

        serve(addr.clone(), protocol.clone(), self.app.clone(), self.handler.clone(), self.error_handler.clone());
        Ok(())
    }
}
