extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

fn handler_root(_app: Arc<App>, _req: Request) -> HandlerResult {
    println!("access: root");
    return Response::text("Hello, world (root)").render();
}

fn handler_wait(_app: Arc<App>, _req: Request) -> HandlerResult {
    // Wait 2 minutes.
    println!("access: wait");
    std::thread::sleep(std::time::Duration::from_secs(120));
    return Response::text("Hello, world (wait)").render();
}

fn main() {
    let app = App::from_config(ZirconConfig::dev());

    let mut router = Router::new();
    router.get("/", handler_root);
    router.get("/wait", handler_wait);

    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Access http://127.0.0.1:3000/");
    server.http(&addr).unwrap();
}
