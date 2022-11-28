use std::{collections::HashMap, sync::Arc};

use crate::{
    messages::{Connect, DataUpdate, SiriUpdate},
    objects::{Line, PTData, VehicleJourney},
};
use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;

pub struct CentralDispatch {
    pub sessions: Vec<Recipient<DataUpdate>>,
    pub pt_data: Option<Arc<crate::PTData>>,
    pub line_referential: Arc<HashMap<String, crate::LineReference>>,
}

impl Actor for CentralDispatch {
    type Context = Context<Self>;
}

impl Handler<Connect> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        tracing::info!("New watcher");
        self.sessions.push(msg.addr.clone());
        if let Some(pt) = &self.pt_data {
            msg.addr.do_send(DataUpdate {
                pt_data: pt.clone(),
            });
        }
    }
}

impl Handler<SiriUpdate> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: SiriUpdate, _ctx: &mut Self::Context) {
        tracing::info!("Fresh SIRI data with {} vehicle journeys", msg.vjs.len());
        let pt_data = Arc::new(self.join_siri_and_theorical(msg.vjs));
        self.pt_data = Some(pt_data.clone());
        for session in &self.sessions {
            session.do_send(DataUpdate {
                pt_data: pt_data.clone(),
            });
        }
    }
}

impl CentralDispatch {
    fn join_siri_and_theorical(&mut self, vjs: Vec<EstimatedVehicleJourney>) -> PTData {
        let mut lines = HashMap::new();
        for vj in vjs {
            if let Some(line_reference) = self.line_referential.get(&vj.line_ref.value) {
                lines
                    .entry(line_reference.id.clone())
                    .or_insert_with(|| Line {
                        reference: line_reference.clone(),
                        vjs: vec![],
                    })
                    .vjs
                    .push(VehicleJourney::from(vj));
            } else {
                tracing::warn!(
                    "Could not find {} line_ref in the static data",
                    vj.line_ref.value
                );
            }
        }

        PTData { lines }
    }
}
/*
{

}*/
