mod source;
mod sources;

use std::sync::{RwLock, RwLockWriteGuard};

use handlebars::Handlebars;

use config::Mode;

pub use self::source::{Source, SourceError};
pub use self::sources::directory::DirectorySource;

/// The handlebars template engine
pub struct HandlebarsEngine {
    pub sources: Vec<Box<Source + Send + Sync>>,
    pub registry: RwLock<Box<Handlebars>>,
    pub mode: Mode,
}

impl HandlebarsEngine {
    /// create a handlebars template engine
    pub fn new(mode: Mode) -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(Handlebars::new())),
            mode: mode,
        }
    }

    /// create a handlebars template engine from existed handlebars registry
    pub fn from(reg: Handlebars, mode: Mode) -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(reg)),
            mode: mode,
        }
    }

    /// add a template source
    pub fn add(&mut self, source: Box<Source + Send + Sync>) {
        self.sources.push(source);
    }

    /// load template from registered sources
    pub fn reload(&self) -> Result<(), SourceError> {
        let mut hbs = self.handlebars_mut();
        hbs.clear_templates();
        for s in self.sources.iter() {
            try!(s.load(&mut hbs))
        }
        Ok(())
    }

    /// access internal handlebars registry, useful to register custom helpers
    pub fn handlebars_mut(&self) -> RwLockWriteGuard<Box<Handlebars>> {
        self.registry.write().unwrap()
    }
}
