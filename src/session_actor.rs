use crate::central_dispatch::CentralDispatch;
use crate::messages::{Connect, UpdateVJs};
use actix::prelude::*;
use actix_web_actors::ws;
pub struct SessionActor {
    pub central: Addr<CentralDispatch>,
}

impl Actor for SessionActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("New session actor started");
        self.central.do_send(Connect {
            addr: ctx.address().recipient(),
        })
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("Session actor is stopped");
    }
}

impl Handler<UpdateVJs> for SessionActor {
    type Result = ();

    fn handle(&mut self, msg: UpdateVJs, ctx: &mut Self::Context) {
        let len = msg.vjs.len();
        let text = format!(
            r#"
<div id="messages">
    Nous avons <b>{len}</b> vehicle journeys
</div>
"#
        );
        ctx.text(text)
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SessionActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
