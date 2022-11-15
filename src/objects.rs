use std::collections::HashMap;

use siri_lite::service_delivery::EstimatedVehicleJourney;

pub struct PTData {
    pub lines: HashMap<String, Line>,
}

pub struct Line {
    pub name: String,
    pub id: String,
    pub mode: String, // the mode could be different for the lines's VJ, but we'll consider this to be the commercial mode for the moment

    pub vjs: Vec<EstimatedVehicleJourney>,
}
