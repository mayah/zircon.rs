extern crate cookie;
extern crate hyper;
extern crate zircon;

use std::sync::Arc;
use zircon::extensions::cookie::{GetCookieJar, SetCookieJar};
use zircon::prelude::*;

type App = ZirconDefaultApp<()>;

fn handler_cookie_show(_app: Arc<App>, req: Request) -> HandlerResult {
    let jar = req.cookie_jar();

    let mut foo = String::new();
    let mut bar = String::new();

    if let Some(c) = jar.get("foo") {
        foo = c.value().to_string();
    }
    if let Some(c) = jar.get("bar") {
        bar = c.value().to_string();
    }

    let s = format!("foo={}, bar={}", foo, bar);
    return Response::text(format!("Hello, world: {}", s)).render();
}

fn handler_cookie_set(_app: Arc<App>, _req: Request) -> HandlerResult {
    // NOTE: This cookie is not signed!
    let mut jar = cookie::CookieJar::new();
    jar.add(cookie::Cookie::new("foo", "baz"));
    jar.add(cookie::Cookie::new("bar", "quux"));
    return Response::text("set")
        .with_cookie_jar(&jar)
        .render();
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler_cookie_show);
    router.get("/set", handler_cookie_set);

    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("access http://localhost:3000 and http://localhost:3000/set");
    server.http(&addr).unwrap();
}
