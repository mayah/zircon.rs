use std::sync::Arc;
use prelude::*;
use renderers::RenderFile;

/// SingleFileHandler is a handler to return a single file.
/// This is a handy way to handle "favicon.ico".
pub struct SingleFileHandler {
    filepath: String,
}

impl SingleFileHandler {
    pub fn new<T: Into<String>>(filepath: T) -> SingleFileHandler {
        SingleFileHandler {
            filepath: filepath.into(),
        }
    }
}

impl<A: ZirconApp> Handler<A> for SingleFileHandler {
    fn handle(&self, _app: Arc<A>, _req: Request) -> HandlerResult {
        Response::new().render_file(&self.filepath)
    }
}
