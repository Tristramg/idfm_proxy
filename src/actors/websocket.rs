use crate::actors::CentralDispatch;
use crate::messages::{Connect, DataUpdate};
use crate::templates::TEMPLATES;
use actix::prelude::*;
use actix_web_actors::ws;
pub struct SessionActor {
    pub central: Addr<CentralDispatch>,
    pub watching: Watching,
}

pub enum Watching {
    Index,
    Line(String),
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
                let text = TEMPLATES
                    .render(
                        "line_list.html",
                        &tera::Context::from_serialize(&template).unwrap(),
                    )
                    .unwrap();
                ctx.text(text)
            }
            Watching::Line(ref line_ref) => {
                let l = msg.pt_data.lines.get(line_ref);
                if let Some(line) = l {
                    let text = TEMPLATES
                        .render("line.html", &tera::Context::from_serialize(line).unwrap())
                        .unwrap();
                    ctx.text(text)
                } else {
                    // 404
                    let mut context = tera::Context::new();
                    context.insert("line_ref", &line_ref.as_str());
                    let text = TEMPLATES.render("line_not_found.html", &context).unwrap();
                    ctx.text(text)
                }
            }
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
