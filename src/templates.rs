use std::collections::HashMap;

use serde::Serialize;
use tera::Tera;

#[derive(Serialize)]
pub struct Lines<'a> {
    pub lines: &'a HashMap<String, crate::Line>,
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.full_reload().expect("unable to full reload templates");
        tera
    };
}
