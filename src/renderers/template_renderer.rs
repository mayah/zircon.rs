use std::error::Error;

use futures::{Future, IntoFuture};
use handlebars::TemplateRenderError;
use hyper::header::{ContentLength, ContentType};
use prelude::*;
use serde_json::value::Value as Json;

use config::Mode;

/// RenderTemplate provides a way to render html template.
pub trait RenderTemplate {
    fn render_html_template(self, engine: &HandlebarsEngine, name: &str, data: &Json) -> HandlerResult;
}

impl RenderTemplate for Response {
    fn render_html_template(mut self, engine: &HandlebarsEngine, name: &str, data: &Json) -> HandlerResult {
        // If not product mode, always reload templates.
        if engine.mode != Mode::Prod {
            match engine.reload() {
                Ok(_) => (),
                Err(err) => {
                    error!("failed to reload engine: err={}", err)
                }
            }
        }

        let page_result = {
            let hbs = engine.registry.read().unwrap();
            hbs.render(name, data).map_err(TemplateRenderError::from)
        };

        let result = match page_result {
            Ok(page) => {
                self.origin.headers_mut().set(ContentType::html());
                self.origin.headers_mut().set(ContentLength(page.len() as u64));
                self.origin.set_body(page);
                Ok(self)
            }
            Err(e) => {
                return ZirconError::render_error_message(StatusCode::InternalServerError, e.description());
            }
        };

        result.into_future().boxed()
    }
}
