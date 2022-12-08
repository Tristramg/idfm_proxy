use std::sync::Arc;

use crate::status::Status;
use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<DataUpdate>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct SiriUpdate {
    pub vjs: Vec<EstimatedVehicleJourney>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GtfsUpdate {
    pub gtfs: gtfs_structures::Gtfs,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct DataUpdate {
    pub pt_data: Arc<crate::objects::PTData>,
}

#[derive(Message, Clone)]
#[rtype(result = "Arc<Status>")]
pub struct StatusDemand {}
