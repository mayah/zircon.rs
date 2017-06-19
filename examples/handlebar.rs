extern crate cookie;
extern crate futures;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate zircon;

use std::sync::Arc;

use serde_json::value::Value as Json;

use zircon::Mode;
use zircon::prelude::*;
use zircon::templates::DirectorySource;
use zircon::renderers::RenderTemplate;

struct ServerData {
    te: HandlebarsEngine,
}

type App = ZirconDefaultApp<ServerData>;

fn index_handler(app: Arc<App>, _req: Request) -> HandlerResult {
    let data = json!({});
    return Response::new().render_html_template(&app.server_data.te, "index", &data);
}

fn user_handler(app: Arc<App>, _req: Request) -> HandlerResult {
    let mut tree = serde_json::Map::<String, Json>::new();
    tree.insert("user".to_string(), Json::String("foo".to_string()));
    let data = Json::Object(tree);

    return Response::new().render_html_template(&app.server_data.te, "user", &data);
}

fn main() {
    let mut hbse = HandlebarsEngine::new(Mode::Dev);
    hbse.add(Box::new(DirectorySource::new("examples/templates/form", ".hbs")));
    hbse.reload().unwrap();

    let mut router = Router::new();
    router.get("/", index_handler);
    router.get("/user", user_handler);

    let app = App::from_config(ZirconConfig::dev())
        .with_server_data(ServerData {
            te: hbse,
        });

    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Server running at http://localhost:3000/");
    server.http(&addr).unwrap();
}
