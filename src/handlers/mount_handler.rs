use prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct MountHandler<A: ZirconApp, H: Handler<A>> {
    mount_point: String,
    next: H,
    _p: PhantomData<A>,
}

impl<A: ZirconApp, H: Handler<A>> MountHandler<A, H> {
    pub fn new<S: Into<String>>(mount_point: S, handler: H) -> MountHandler<A, H> {
        let mp = mount_point.into();

        assert_eq!(mp.as_bytes()[0], b'/');
        assert_ne!(mp.as_bytes()[mp.as_bytes().len() - 1], b'/');

        MountHandler {
            mount_point: mp,
            next: handler,
            _p: PhantomData,
        }
    }
}

impl<A: ZirconApp, H: Handler<A>> Handler<A> for MountHandler<A, H> {
    fn handle(&self, app: Arc<A>, mut req: Request) -> HandlerResult {
        // TODO(mayah): new_path should be a substring of req.path.
        // So, can't we use &str instead of String?
        let new_path = {
            let path = req.path();
            if !path.starts_with(&self.mount_point) {
                panic!("path didn't match");
            }
            path[self.mount_point.len()..].to_string()
        };

        req.set_modified_path(Some(new_path));
        self.next.handle(app, req)
    }
}
