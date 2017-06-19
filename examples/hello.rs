extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

fn handler(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::text("Hello, world foo!").render();
}

fn main() {
    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, handler);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Access http://127.0.0.1:3000/");
    server.http(&addr).unwrap();
}
