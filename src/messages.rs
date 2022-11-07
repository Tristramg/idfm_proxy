use std::sync::Arc;

use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<UpdateVJs>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct UpdateVJs {
    pub vjs: Arc<Vec<EstimatedVehicleJourney>>,
}
