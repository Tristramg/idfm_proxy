use std::collections::HashMap;

use serde::Deserialize;
use siri_lite::service_delivery::EstimatedVehicleJourney;

pub struct PTData {
    pub lines: HashMap<String, Line>,
}

pub struct Line {
    pub reference: LineReference,
    pub vjs: Vec<EstimatedVehicleJourney>,
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
