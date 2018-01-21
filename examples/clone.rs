extern crate zircon;

use zircon::prelude::*;
use std::sync::Arc;

struct Data {
    x: usize,
}

type App = ZirconDefaultApp<Data>;

fn handler(_app: Arc<App>, _req: Request) -> HandlerResult {
    return Response::text("Hello, world foo!").render();
}

fn main() {
    let server_data = Data {
        x: 10,
    };

    let app = App::from_config(ZirconConfig::dev()).with_server_data(server_data);
    let server = Zircon::new(app, handler);

    println!("x={}", server.app().server_data.x);

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Access http://127.0.0.1:3000/");
    server.http(&addr).unwrap();
}
