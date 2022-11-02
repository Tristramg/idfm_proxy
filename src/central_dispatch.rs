use crate::messages::{Broadcast, Connect, Recieved};
use actix::prelude::*;
pub struct CentralDispatch {
    pub sessions: Vec<Recipient<Broadcast>>,
}

impl Actor for CentralDispatch {
    type Context = Context<Self>;
}

impl Handler<Recieved> for CentralDispatch {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Recieved, _ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received: {}", msg.msg);
        self.sessions.iter().for_each(|session| {
            session.do_send(Broadcast {
                msg: msg.msg.clone(),
            })
        });
        Ok(true)
    }
}

impl Handler<Connect> for CentralDispatch {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Nouvel abonné !");
        self.sessions.push(msg.addr.clone());
    }
}
