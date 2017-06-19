extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

struct MyHandler {
    s: String,
}

impl zircon::Handler<App> for MyHandler {
    fn handle(&self, _app: Arc<App>, _req: Request) -> HandlerResult {
        let x = format!("Hello, world {}", self.s);
        return Response::text(x).render();
    }
}

fn main() {
    let handler = MyHandler { s: "hoge".to_string() };

    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, handler);
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.http(&addr).unwrap();
}
