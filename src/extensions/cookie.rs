use prelude::*;

use cookie;
use hyper;
use hyper::header::SetCookie;

pub trait GetCookieJar {
    fn cookie_jar(&self) -> cookie::CookieJar;
}

pub trait SetCookieJar {
    fn with_cookie_jar(self, jar: &cookie::CookieJar) -> Self;
}

impl GetCookieJar for Request {
    fn cookie_jar(&self) -> cookie::CookieJar {
        let mut jar = cookie::CookieJar::new();

        if let Some(cookies) = self.headers().get::<hyper::header::Cookie>() {
            for cs in cookies.iter() {
                let c = cookie::Cookie::new(cs.0.to_string(), cs.1.to_string());
                jar.add_original(c.into_owned());
            }
        }

        jar
    }
}

impl SetCookieJar for Response {
    fn with_cookie_jar(mut self, jar: &cookie::CookieJar) -> Response {
        self.origin.headers_mut().set(SetCookie(jar.delta().into_iter().map(|c| {
            format!("{}", c)
        }).collect()));
        self
    }
}
