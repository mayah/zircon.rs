use std::fs::File;
use std::io::Read;
use std::path::Path;

use futures::{Future, IntoFuture};
use hyper::header::{CacheControl, CacheDirective};

use prelude::*;

/// RenderFile sends a file to a user.
// TODO(mayah): currently this is blocking operation, which can eliminate tokio
// advantage. So, we cannot send a large file to a user.
// We need to mimic something like tk-sendfile crate.
pub trait RenderFile {
    fn render_file<P: AsRef<Path>>(self, path: P) -> HandlerResult;
}

impl RenderFile for Response {
    fn render_file<P: AsRef<Path>>(mut self, path: P) -> HandlerResult {
        let mut buf = Vec::<u8>::new();

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(err) => {
                return ZirconError::IoError(err).render()
            }
        };

        match file.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                return ZirconError::IoError(err).render()
            }
        }

        if !self.origin.headers().has::<CacheControl>() {
            // Default is 1day.
            self.origin.headers_mut().set(CacheControl(vec![CacheDirective::MaxAge(86400u32)]));
        }

        self.origin.set_body(buf);
        Ok(self).into_future().boxed()
    }
}
