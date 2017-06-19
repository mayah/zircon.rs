use cookie;

pub trait SessionConverter : Sized + 'static {
    fn session_key() -> &'static str;
    fn serialize_for_session(self) -> String;
    fn deserialize_for_session(&str) -> Option<Self>;
}

pub trait GetSession {
    fn session<S: SessionConverter>(&self) -> Option<S>;
}

impl<'a> GetSession for cookie::SignedJar<'a> {
    fn session<S: SessionConverter>(&self) -> Option<S> {
        if let Some(c) = self.get(S::session_key()) {
            let value = c.value();
            return S::deserialize_for_session(value);
        }

        // Nothing found.
        None
    }
}

pub trait SetSession {
    fn set_session<S: SessionConverter>(&mut self, S);
    fn remove_session<S: SessionConverter>(&mut self);
}

impl<'a> SetSession for cookie::SignedJar<'a> {
    fn set_session<S: SessionConverter>(&mut self, session: S) {
        // TODO(mayah): We'd like to use secure() for https site.
        // TODO(mayah): Better to have same_site(cookie::SmaeSite::Strict)?
        let ck = cookie::Cookie::build(S::session_key(), session.serialize_for_session())
            .path("/")
            .http_only(true)
            .finish();
        self.add(ck);
    }

    fn remove_session<S: SessionConverter>(&mut self) {
        // Note: self.remove(ck) didn't work well. It didn't remove cookie in chrome.
        let ck = cookie::Cookie::build(S::session_key(), "")
            .path("/")
            .http_only(true)
            .finish();
        self.add(ck);
    }
}
