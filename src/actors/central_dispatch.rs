use crate::{
    messages::{Connect, DataUpdate, StatusDemand},
    status::Status,
};
use actix::prelude::*;
use std::sync::Arc;

pub struct CentralDispatch {
    pub sessions: Vec<Recipient<DataUpdate>>,
}

impl Actor for CentralDispatch {
    type Context = Context<Self>;
}

impl Handler<Connect> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) {
        tracing::info!("New watcher");
        self.sessions.push(msg.addr);
    }
}

impl Handler<DataUpdate> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: DataUpdate, _ctx: &mut Self::Context) {
        tracing::info!("dispatching data to all watchers");
        for session in &self.sessions {
            tracing::info!("whee a session");
            session.do_send(msg.clone());
        }
    }
}

impl Handler<StatusDemand> for CentralDispatch {
    type Result = Arc<Status>;

    fn handle(&mut self, _msg: StatusDemand, _ctx: &mut Self::Context) -> Self::Result {
        Arc::new(Status {
            nb_open_connections: self.sessions.len(),
        })
    }
}
