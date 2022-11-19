use crate::central_dispatch::CentralDispatch;
use crate::messages::{Connect, DataUpdate};
use actix::prelude::*;
use actix_web_actors::ws;
use askama::Template;
pub struct SessionActor {
    pub central: Addr<CentralDispatch>,
    pub watching: Watching,
}

pub enum Watching {
    Index,
    Line(String),
    Point(String),
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

impl Handler<DataUpdate> for SessionActor {
    type Result = ();

    fn handle(&mut self, msg: DataUpdate, ctx: &mut Self::Context) {
        match self.watching {
            Watching::Index => {
                let template = crate::templates::Lines {
                    lines: &msg.pt_data.lines,
                };
                ctx.text(template.render().unwrap())
            }
            Watching::Line(ref line_ref) => {
                let l = msg.pt_data.lines.get(line_ref);
                if let Some(line) = l {
                    let line_template = crate::templates::Line { line };
                    ctx.text(line_template.render().unwrap())
                } else {
                    // 404
                    let line_not_found_template = crate::templates::LineNotFound {
                        line_ref: line_ref.as_str(),
                    };
                    ctx.text(line_not_found_template.render().unwrap())
                }
            }
            Watching::Point(_) => unimplemented!("point not implemented"),
        }
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
