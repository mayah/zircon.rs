extern crate getopts;
extern crate zircon;

use zircon::prelude::*;
use zircon::handlers::StaticFileHandler;

type App = ZirconDefaultApp<()>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.reqopt("", "dir", "serving file directory", "DIR");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let directory = match matches.opt_str("dir") {
        Some(x) => x,
        None => {
            panic!("--dir is required");
        },
    };

    let handler = StaticFileHandler::new(directory);
    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, handler);
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.http(&addr).unwrap();
}
