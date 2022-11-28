use itertools::Itertools;
use std::collections::HashMap;

use askama_actix::Template;
use serde::Deserialize;
use siri_lite::{service_delivery::EstimatedVehicleJourney, shared::DateTime};

pub struct PTData {
    pub lines: HashMap<String, Line>,
}

pub struct Line {
    pub reference: LineReference,
    pub vjs: Vec<VehicleJourney>,
}

// Struct to deserialize the data from https://data.iledefrance-mobilites.fr/explore/dataset/referentiel-des-lignes
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineReference {
    #[serde(rename = "ID_Line")]
    pub id: String,
    #[serde(rename = "Name_Line")]
    pub name: String,
    #[serde(rename = "ShortName_Line")]
    pub short_name: String,
    #[serde(rename = "TransportMode")]
    pub mode: String,
    #[serde(rename = "TransportSubmode")]
    pub sub_mode: String,
    pub operator_name: String,
    pub network_name: String,
    #[serde(rename = "ColourWeb_hexa")]
    pub background_color: String,
    #[serde(rename = "TextColourWeb_hexa")]
    pub text_color: String,
}

pub struct VehicleJourney {
    pub origin: String,
    pub destination: String,
    pub estimated_calls: Vec<EstimatedCall>,
}

impl From<EstimatedVehicleJourney> for VehicleJourney {
    fn from(vj: EstimatedVehicleJourney) -> Self {
        Self {
            origin: vj.origin_name.iter().map(|name| &name.value).join(", "),
            destination: vj
                .destination_name
                .iter()
                .map(|name| &name.value)
                .join(", "),
            estimated_calls: vj
                .estimated_calls
                .estimated_call
                .iter()
                .map(|e| EstimatedCall::from(e))
                .collect(),
        }
    }
}

pub enum CallStatus<'a> {
    None,
    OnTime(&'a DateTime),
    Delayed(&'a DateTime, &'a DateTime),
}
#[derive(Template)]
#[template(path = "estimated_call.html")]
pub struct EstimatedCall {
    pub expected_arrival_time: Option<DateTime>,
    pub aimed_arrival_time: Option<DateTime>,
    pub expected_departure_time: Option<DateTime>,
    pub aimed_departure_time: Option<DateTime>,
}

impl EstimatedCall {
    pub fn arrival_status(&self) -> CallStatus {
        match (&self.expected_departure_time, &self.aimed_arrival_time) {
            (None, None) => CallStatus::None,
            (Some(expected), None) => CallStatus::OnTime(expected),
            (None, Some(aimed)) => CallStatus::OnTime(aimed),
            (Some(expected), Some(aimed)) => {
                if expected == aimed {
                    CallStatus::OnTime(expected)
                } else {
                    CallStatus::Delayed(aimed, expected)
                }
            }
        }
    }
}

impl From<&siri_lite::service_delivery::EstimatedCall> for EstimatedCall {
    fn from(siri_estimated_call: &siri_lite::service_delivery::EstimatedCall) -> Self {
        Self {
            expected_arrival_time: siri_estimated_call.expected_arrival_time.clone(),
            aimed_arrival_time: siri_estimated_call.aimed_arrival_time.clone(),
            expected_departure_time: siri_estimated_call.expected_departure_time.clone(),
            aimed_departure_time: siri_estimated_call.aimed_departure_time.clone(),
        }
    }
}

pub mod filters {
    pub fn time(dt: &siri_lite::shared::DateTime) -> ::askama::Result<String> {
        Ok(dt.0.format("%H:%M:%S").to_string())
    }
}
