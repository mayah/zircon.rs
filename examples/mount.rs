extern crate zircon;

use zircon::prelude::*;
use zircon::handlers::MountHandler;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

fn handler_root(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::text("Hello, world (root). Try to access /user/foo, /user/bar, etc.").render();
}

// request path without /user should be shown as path.
fn handler_show_path(_app: Arc<App>, req: Request) -> HandlerResult {
    return Response::text(format!("path={}", req.path())).render();
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler_root);
    router.get("/user/**", MountHandler::new("/user", handler_show_path));

    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.http(&addr).unwrap();
}
