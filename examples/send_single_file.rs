extern crate getopts;
extern crate zircon;

use zircon::prelude::*;
use zircon::handlers::SingleFileHandler;

type App = ZirconDefaultApp<()>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.reqopt("f", "file", "sending file name", "FILENAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let filename = match matches.opt_str("f") {
        Some(x) => x,
        None => {
            panic!("--file is required");
        },
    };

    let handler = SingleFileHandler::new(filename);
    let app = App::from_config(ZirconConfig::dev());
    let server = Zircon::new(app, handler);
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.http(&addr).unwrap();
}
