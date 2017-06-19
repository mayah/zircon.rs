extern crate cookie;
extern crate zircon;

use std::sync::Arc;

use zircon::prelude::*;
use zircon::extensions::cookie::{GetCookieJar, SetCookieJar};
use zircon::extensions::session::{GetSession, SetSession, SessionConverter};

struct ServerData {
    signing_key: cookie::Key,
}
type App = ZirconDefaultApp<ServerData>;

// ----------------------------------------------------------------------

pub struct SessionData {
    id: i64,
    name: String,
}

impl SessionConverter for SessionData {
    fn session_key() -> &'static str { "session" }

    fn serialize_for_session(self) -> String {
        format!("{}/{}", self.id, self.name)
    }

    fn deserialize_for_session(s: &str) -> Option<SessionData> {
        if let Some(pos) = s.find('/') {
            let id_str = &s[..pos];
            let name_str = &s[(pos+1)..];
            if let Ok(id) = id_str.parse() {
                return Some(SessionData {
                    id: id,
                    name: name_str.to_string(),
                });
            }
        }

        return None;
    }
}

// ----------------------------------------------------------------------

fn handler_session_show(app: Arc<App>, req: Request) -> HandlerResult {
    let mut root_jar = req.cookie_jar();
    let jar = root_jar.signed(&app.server_data.signing_key);

    let session = jar.session::<SessionData>();
    if session.is_none() {
        return Response::text("no session").render();
    }

    let session = session.unwrap();
    let s = format!("id={} name={}", session.id, session.name);
    return Response::text(format!("session: {}", s)).render();
}

fn handler_session_set(app: Arc<App>, req: Request) -> HandlerResult {
    let mut root_jar = req.cookie_jar();
    {
        let mut jar = root_jar.signed(&app.server_data.signing_key);
        jar.set_session(SessionData {
            id: 82,
            name: "kotori".to_string(),
        });
    }
    return Response::text("set done")
        .with_cookie_jar(&root_jar)
        .render();
}

fn handler_session_invalidate(app: Arc<App>, req: Request) -> HandlerResult {
    let mut root_jar = req.cookie_jar();
    {
        let mut jar = root_jar.signed(&app.server_data.signing_key);
        jar.set_session(SessionData {
            id: -1,
            name: "unused".to_string(),
        });
    }

    return Response::text("set done")
        .with_cookie_jar(&root_jar)
        .render();
}

fn main() {
    let signing_key = b"testtesttesttesttesttesttesttest";

    let mut router = Router::new();
    router.get("/", handler_session_show);
    router.get("/set", handler_session_set);
    router.get("/invalidate", handler_session_invalidate);

    let app = App::from_config(ZirconConfig::dev())
        .with_server_data(ServerData {
            signing_key: cookie::Key::from_master(signing_key),
        });
    let server = Zircon::new(app, router);
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("access http://localhost:3000");
    server.http(&addr).unwrap();
}
