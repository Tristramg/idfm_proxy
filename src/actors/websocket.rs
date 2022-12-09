use crate::actors::{CentralDispatch, DataStore, Templates};
use crate::messages::*;
use actix::prelude::*;
use actix_web_actors::ws;
pub struct SessionActor {
    pub central: Addr<CentralDispatch>,
    pub data_store: Addr<DataStore>,
    pub templates: Addr<Templates>,
    pub watching: Watching,
}

pub enum Watching {
    Index,
    Line(String),
}

impl SessionActor {
    fn render(
        &self,
        ws_ctx: &mut ws::WebsocketContext<Self>,
        template: &'static str,
        context: tera::Context,
    ) {
        self.templates
            .send(RenderTemplate { template, context })
            .into_actor(self)
            .map(|result, _act, ctx| {
                if let Ok(Ok(html)) = result {
                    ctx.text(html)
                } else {
                    tracing::error!("When rendering template template {:?}", result);
                }
            })
            .wait(ws_ctx);
    }
}

impl Actor for SessionActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("New session actor started");
        self.central.do_send(Connect {
            addr: ctx.address().recipient(),
        });

        self.data_store
            .send(CurrentPTData {})
            .into_actor(self)
            .map(|result, _act, ctx| {
                if let Ok(Some(pt_data)) = result {
                    ctx.notify(DataUpdate { pt_data });
                }
            })
            .wait(ctx);
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
                let mut context = tera::Context::new();
                context.insert("lines", &msg.pt_data.lines);
                self.render(ctx, "line_list.html", context);
            }
            Watching::Line(ref line_ref) => {
                let l = msg.pt_data.lines.get(line_ref);
                if let Some(line) = l {
                    self.render(
                        ctx,
                        "line.html",
                        tera::Context::from_serialize(line).unwrap(),
                    );
                } else {
                    // 404
                    let mut context = tera::Context::new();
                    context.insert("line_ref", &line_ref.as_str());
                    self.render(ctx, "line_not_found.html", context);
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
