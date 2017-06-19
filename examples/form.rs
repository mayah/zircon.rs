extern crate futures;
extern crate zircon;

extern crate serde;
#[macro_use] extern crate serde_json;

use std::sync::Arc;

use futures::Future;
use serde_json::value::Value as Json;

use zircon::Mode;
use zircon::prelude::*;
use zircon::renderers::RenderTemplate;
use zircon::templates::DirectorySource;

struct ServerData {
    te: HandlebarsEngine,
}

type App = ZirconDefaultApp<ServerData>;

fn handler_root(app: Arc<App>, _req: Request) -> HandlerResult {
    let data = json!({});
    return Response::new().render_html_template(&app.server_data.te, "index", &data);
}

fn handler_form(app: Arc<App>, req: Request) -> HandlerResult {
    let (_header, body) = req.deconstruct();
    Box::new(body.parse_form_body().and_then(move |form| {
        let mut map = serde_json::map::Map::new();
        let user = form.find_first("user").map(|x| x.to_string());
        if let Some(user) = user {
            map.insert("user".to_string(), Json::String(user));
        }

        let data = Json::Object(map);
        return Response::new().render_html_template(&app.server_data.te, "form", &data);
    }))
}

fn main() {
    let mut hbse = HandlebarsEngine::new(Mode::Dev);
    hbse.add(Box::new(DirectorySource::new("examples/templates/form", ".hbs")));
    hbse.reload().unwrap();

    let mut router = Router::new();
    router.get("/", handler_root);
    router.post("/form", handler_form);

    let app = App::from_config(ZirconConfig::dev())
        .with_server_data(ServerData {
            te: hbse,
        });

    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Access POST http://127.0.0.1:3000/");
    server.http(&addr).unwrap();
}
