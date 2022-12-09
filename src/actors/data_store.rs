use crate::messages::*;
use crate::objects::{Line, PTData, VehicleJourney};
use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;
use std::{collections::HashMap, sync::Arc};

pub struct DataStore {
    pub central_dispatch: Addr<super::CentralDispatch>,
    pub pt_data: Option<Arc<PTData>>,
    pub line_referential: Arc<HashMap<String, crate::LineReference>>,
    pub stop_referential: Arc<HashMap<String, crate::StopReference>>,
}

impl Actor for DataStore {
    type Context = Context<Self>;
}

impl Handler<SiriUpdate> for DataStore {
    type Result = ();

    fn handle(&mut self, msg: SiriUpdate, _ctx: &mut Self::Context) {
        tracing::info!("Fresh SIRI data with {} vehicle journeys", msg.vjs.len());
        let pt_data = Arc::new(self.join_siri_and_theorical(msg.vjs));
        self.pt_data = Some(pt_data.clone());
        self.central_dispatch.do_send(DataUpdate {
            pt_data: pt_data.clone(),
        });
    }
}

impl Handler<CurrentPTData> for DataStore {
    type Result = Option<Arc<crate::objects::PTData>>;

    fn handle(&mut self, _msg: CurrentPTData, _ctx: &mut Self::Context) -> Self::Result {
        self.pt_data.clone()
    }
}

impl DataStore {
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
                    .push(VehicleJourney::from(vj).patch_name(&self.stop_referential));
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
