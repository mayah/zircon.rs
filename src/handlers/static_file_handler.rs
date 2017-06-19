use prelude::*;
use renderers::RenderFile;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std;

/// StaticFileHandler serves files in a given root directory.
pub struct StaticFileHandler {
    root: PathBuf,
}

impl StaticFileHandler {
    pub fn new<P: AsRef<Path>>(root: P) -> StaticFileHandler {
        StaticFileHandler {
            root: root.as_ref().to_path_buf(),
        }
    }
}

// TODO(mayah): Currently this doesn't allow '..' Anything else? Can we confirm this is safe?
fn is_safe_path<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().components().all(|c| match c {
        std::path::Component::CurDir | std::path::Component::Normal(_) => true,
        _ => false
    })
}

impl<A: ZirconApp> Handler<A> for StaticFileHandler {
    fn handle(&self, _app: Arc<A>, req: Request) -> HandlerResult {
        let path: &str = req.path();
        assert_eq!(path.chars().nth(0).unwrap(), '/');
        let path = match path {
            "/" => "index.html",
            _ => &path[1..],
        };

        if !is_safe_path(path) {
            return ZirconError::render_error_status(StatusCode::BadRequest);
        }

        let path = &self.root.join(path);
        match std::fs::metadata(path) {
            Ok(ref attr) if attr.is_file() => {
                return Response::new().render_file(path);
            },
            Err(ref e) if e.kind() != std::io::ErrorKind::NotFound => {
                debug!("Error getting metadata for file '{:?}': {:?}", path, e);
                return ZirconError::render_error_message(StatusCode::InternalServerError, "failed to get file metadata");
            },
            _ => {
                return ZirconError::render_error_status(StatusCode::NotFound);
            }
        }
    }
}
