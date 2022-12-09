use actix::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

use crate::messages::RenderTemplate;

fn to_paris_time(date: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match date.as_str() {
        Some(date) => {
            let dt = chrono::DateTime::parse_from_rfc3339(date)
                .map_err(|e| tera::Error::msg(format!("could not parse date {date}: {e}")))?;
            let paris_time = dt.with_timezone(&chrono_tz::Europe::Paris);
            Ok(Value::String(paris_time.format("%H:%M:%S").to_string()))
        }
        _ => Ok(Value::Null),
    }
}

pub struct Templates {
    tera: tera::Tera,
}

impl Templates {
    pub fn new() -> Self {
        let mut tera = match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.register_filter("to_paris_time", to_paris_time);
        Self { tera }
    }
}

impl Actor for Templates {
    type Context = Context<Self>;
}

impl Handler<RenderTemplate<'_>> for Templates {
    type Result = tera::Result<String>;
    fn handle(&mut self, msg: RenderTemplate, _ctx: &mut Self::Context) -> Self::Result {
        self.tera.render(msg.template, &msg.context)
    }
}
