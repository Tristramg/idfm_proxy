use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tera::*;

#[derive(Serialize)]
pub struct Lines<'a> {
    pub lines: &'a HashMap<String, crate::objects::Line>,
}

fn to_paris_time(date: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    match date.as_str() {
        Some(date) => {
            let dt = chrono::DateTime::parse_from_rfc3339(&date.to_string())
                .map_err(|e| Error::msg(format!("could not parse date {date}: {e}")))?;
            let paris_time = dt.with_timezone(&chrono_tz::Europe::Paris);
            Ok(Value::String(paris_time.format("%H:%M:%S").to_string()))
        }
        _ => Ok(Value::Null),
    }
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
        tera.register_filter("to_paris_time", &to_paris_time);
        tera
    };
}
