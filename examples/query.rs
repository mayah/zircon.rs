extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

type App = ZirconDefaultApp<()>;

fn handler(_app: Arc<App>, req: Request) -> HandlerResult {
    let query = req.parse_query();

    let s = format!("foo={:?}", query.find_first("foo"));
    return Response::text(s).render();
}

fn main() {
    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, handler);
    let addr = "127.0.0.1:3000".parse().unwrap();

    println!("access http://localhost:3000/?foo=bar");
    server.http(&addr).unwrap();
}
