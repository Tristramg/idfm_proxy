use itertools::Itertools;
use serde::{Deserialize, Serialize};
use siri_lite::{service_delivery::EstimatedVehicleJourney, shared::DateTime};
use std::{cmp::Ordering, collections::HashMap, sync::Arc};

pub struct PTData {
    pub lines: HashMap<String, Line>,
}

#[derive(Serialize)]
pub struct Line {
    pub reference: LineReference,
    pub vjs: Vec<VehicleJourney>,
}

// Struct to deserialize the data from https://data.iledefrance-mobilites.fr/explore/dataset/referentiel-des-lignes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct LineReference {
    #[serde(rename(deserialize = "ID_Line"))]
    pub id: String,
    #[serde(rename(deserialize = "Name_Line"))]
    pub name: String,
    #[serde(rename(deserialize = "ShortName_Line"))]
    pub short_name: String,
    #[serde(rename(deserialize = "TransportMode"))]
    pub mode: String,
    #[serde(rename(deserialize = "TransportSubmode"))]
    pub sub_mode: String,
    pub operator_name: String,
    pub network_name: String,
    #[serde(rename(deserialize = "ColourWeb_hexa"))]
    pub background_color: String,
    #[serde(rename(deserialize = "TextColourWeb_hexa"))]
    pub text_color: String,
}

// Struct to deserialize the data from https://prim.iledefrance-mobilites.fr/fr/donnees-statiques/arrets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopReference {
    #[serde(rename(deserialize = "ArRId"))]
    pub id: String,
    #[serde(rename(deserialize = "ArRName"))]
    pub name: String,
}

#[derive(Serialize)]
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
                .map(EstimatedCall::from)
                .sorted()
                .collect(),
        }
    }
}

impl VehicleJourney {
    pub fn patch_name(mut self, stops: &Arc<HashMap<String, crate::StopReference>>) -> Self {
        for call in &mut self.estimated_calls {
            call.patch_name(stops)
        }
        self
    }
}

#[derive(Serialize)]
pub struct EstimatedCall {
    pub expected_arrival_time: Option<DateTime>,
    pub aimed_arrival_time: Option<DateTime>,
    pub expected_departure_time: Option<DateTime>,
    pub aimed_departure_time: Option<DateTime>,
    pub stop_point_ref: String,
    pub stop_name: Option<String>,
}

impl From<&siri_lite::service_delivery::EstimatedCall> for EstimatedCall {
    fn from(siri_estimated_call: &siri_lite::service_delivery::EstimatedCall) -> Self {
        Self {
            expected_arrival_time: siri_estimated_call.expected_arrival_time.clone(),
            aimed_arrival_time: siri_estimated_call.aimed_arrival_time.clone(),
            expected_departure_time: siri_estimated_call.expected_departure_time.clone(),
            aimed_departure_time: siri_estimated_call.aimed_departure_time.clone(),
            stop_point_ref: siri_estimated_call.stop_point_ref.value.clone(),
            stop_name: None,
        }
    }
}

impl EstimatedCall {
    pub fn patch_name(&mut self, stops: &Arc<HashMap<String, crate::StopReference>>) {
        // Sometimes, instead of Q, there is BP, but works the same
        // Source : idfm slack
        let stop_id = &self
            .stop_point_ref
            .replace("STIF:StopPoint:BP:", "STIF:StopPoint:Q:");
        if let Some(stop) = stops.get(stop_id) {
            self.stop_name = Some(stop.name.clone());
        } else {
            tracing::info!(
                "Could not find stop_point_ref {} in stops reference",
                self.stop_point_ref
            )
        }
    }

    pub fn reference_time(&self) -> Option<DateTime> {
        self.expected_arrival_time
            .clone()
            .or_else(|| self.expected_departure_time.clone())
            .or_else(|| self.aimed_arrival_time.clone())
            .or_else(|| self.aimed_departure_time.clone())
    }
}

impl Ord for EstimatedCall {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.reference_time(), other.reference_time()) {
            (Some(a), Some(b)) => a.0.cmp(&b.0),
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for EstimatedCall {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EstimatedCall {
    fn eq(&self, other: &Self) -> bool {
        self.reference_time() == other.reference_time()
    }
}

impl Eq for EstimatedCall {}
