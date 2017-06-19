mod matcher;

use hyper::Method;
use prelude::*;
use std::sync::Arc;

use self::matcher::Matcher;
pub use self::matcher::RouteResult;

pub struct Route<A: ZirconApp> {
    method: Method,
    matcher: Matcher,
    handler: Box<Handler<A>>,
}

pub struct Router<A: ZirconApp> {
    routes: Vec<Route<A>>,
}

impl<A: ZirconApp> Router<A> {
    pub fn new() -> Router<A> {
        Router {
            routes: Vec::new(),
        }
    }

    pub fn add_route<H: Handler<A>>(&mut self, method: Method, path: &str, handler: H) {
        self.routes.push(Route {
            method: method,
            matcher: path.into(),
            handler: Box::new(handler),
        });
    }

    pub fn get<H: Handler<A>>(&mut self, path: &str, handler: H) {
        let route = Route {
            method: Method::Get,
            matcher: path.into(),
            handler: Box::new(handler),
        };

        self.routes.push(route);
    }

    pub fn post<H: Handler<A>>(&mut self, path: &str, handler: H) {
        let route = Route {
            method: Method::Post,
            matcher: path.into(),
            handler: Box::new(handler),
        };

        self.routes.push(route);
    }
}

impl<A: ZirconApp> Handler<A> for Router<A> {
    fn handle(&self, app: Arc<A>, mut req: Request) -> HandlerResult {
        for route in &self.routes {
            if route.method != *req.method() {
                continue;
            }

            if let Some(result) = route.matcher.match_route(req.path()) {
                req.set_params(Some(result));
                return route.handler.handle(app, req);
            }
        }

        // Nothing matched.
        return ZirconError::render_error_status(StatusCode::NotFound);
    }
}
