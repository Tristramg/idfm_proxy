use std::collections::HashMap;

use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "line_index.html")]
pub struct LineIndex {
    pub line_ref: String,
}

#[derive(Template)]
#[template(path = "line_list.html")]
pub struct Lines<'a> {
    pub lines: &'a HashMap<String, crate::Line>,
}

#[derive(Template)]
#[template(path = "line.html")]
pub struct Line<'a> {
    pub line: &'a crate::Line,
}

#[derive(Template)]
#[template(path = "line_not_found.html")]
pub struct LineNotFound<'a> {
    pub line_ref: &'a str,
}

pub mod filters {
    use siri_lite::service_delivery::EstimatedVehicleJourney;

    pub fn route(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        Ok(vj
            .route_ref
            .value
            .as_ref()
            .unwrap_or(&"inconnu".to_string())
            .clone())
    }

    pub fn origin(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        Ok(vj
            .origin_ref
            .value
            .as_ref()
            .unwrap_or(&"inconnu".to_string())
            .clone())
    }

    pub fn direction(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        Ok(vj
            .direction_ref
            .value
            .as_ref()
            .unwrap_or(&"inconnu".to_string())
            .clone())
    }

    pub fn destinations(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.destination_name.iter().map(|d| &d.value), ", ")
    }

    pub fn direction_names(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.direction_name.iter().map(|d| &d.value), ", ")
    }

    pub fn origins(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.origin_name.iter().map(|d| &d.value), ", ")
    }

    pub fn published_line_name(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.published_line_name.iter().map(|d| &d.value), ", ")
    }

    pub fn journey_note(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.journey_note.iter().map(|d| &d.value), ", ")
    }

    pub fn vj_names(vj: &EstimatedVehicleJourney) -> ::askama::Result<String> {
        askama::filters::join(vj.vehicle_journey_name.iter().map(|d| &d.value), ", ")
    }
}
