use std::sync::Arc;

use crate::messages::{Connect, DataUpdate};
use actix::prelude::*;

pub struct CentralDispatch {
    pub sessions: Vec<Recipient<DataUpdate>>,
    pub pt_data: Option<Arc<crate::PTData>>,
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

impl Handler<DataUpdate> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: DataUpdate, _ctx: &mut Self::Context) {
        tracing::info!("Fresh SIRI data with {} lines", msg.pt_data.lines.len());
        self.pt_data = Some(msg.pt_data.clone());
        for session in &self.sessions {
            session.do_send(msg.clone());
        }
    }
}
