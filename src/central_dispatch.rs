use multimap::MultiMap;
use std::sync::Arc;

use crate::messages::{Connect, UpdateVJs};
use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;
pub struct CentralDispatch {
    pub sessions: Vec<Recipient<UpdateVJs>>,
    pub vjs: Option<Arc<MultiMap<String, EstimatedVehicleJourney>>>,
}

impl Actor for CentralDispatch {
    type Context = Context<Self>;
}

impl Handler<Connect> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        tracing::info!("New watcher");
        self.sessions.push(msg.addr.clone());
        if let Some(vjs) = &self.vjs {
            msg.addr.do_send(UpdateVJs { vjs: vjs.clone() });
        }
    }
}

impl Handler<UpdateVJs> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: UpdateVJs, _ctx: &mut Self::Context) {
        tracing::info!("Fresh SIRI data with {} vehicle journeys", msg.vjs.len());
        self.vjs = Some(msg.vjs.clone());
        for session in &self.sessions {
            session.do_send(msg.clone());
        }
    }
}
