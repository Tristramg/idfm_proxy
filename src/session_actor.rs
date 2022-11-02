use crate::central_dispatch::CentralDispatch;
use crate::messages::{Broadcast, Connect, HtmxMessage, Recieved};
use actix::prelude::*;
use actix_web::web::Buf;
use actix_web_actors::ws;
pub struct SessionActor {
    pub central: Addr<CentralDispatch>,
}

impl Actor for SessionActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("We have a new actor !");
        self.central.do_send(Connect {
            addr: ctx.address().recipient(),
        })
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl Handler<Broadcast> for SessionActor {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, ctx: &mut Self::Context) {
        println!("Broadcast reçu ! {}", msg.msg);
        let text = format!(
            r#"
<div id="messages" hx-swap-oob="beforeend">
    {}
</div>
"#,
            msg.msg
        );
        ctx.text(text)
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SessionActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let htmx_message: HtmxMessage = serde_json::from_reader(text.into_bytes().reader())
                    .expect("impossible de lire le message htmx");
                let msg = Recieved {
                    msg: htmx_message.chat_message,
                };
                self.central.do_send(msg);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
