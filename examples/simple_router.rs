extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

fn handler_root(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::text("Hello, world (root)").render();
}

fn handler_user(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::text("Hello, world (user)").render();
}

fn handler_regex(_app: Arc<App>, req: Request) -> HandlerResult {
    let s = format!("Hello, world (regex) : user={:?}", req.param("user"));
    return Response::text(s).render();
}

fn handler_redirect(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::redirect("/").render();
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler_root);
    router.get("/user", handler_user);
    router.get("/regex/:user", handler_regex);
    router.get("/redirect", handler_redirect);

    let app = App::from_config(ZirconConfig::dev());

    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.http(&addr).unwrap();
}
