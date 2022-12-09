use crate::actors::*;
use crate::messages::RenderTemplate;
use actix::prelude::*;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use color_eyre::Result;

#[get("/ws")]
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    central: web::Data<Addr<CentralDispatch>>,
    data_store: web::Data<Addr<DataStore>>,
    templates: web::Data<Addr<Templates>>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("new websocket");

    ws::start(
        SessionActor {
            central: central.as_ref().clone(),
            watching: Watching::Index,
            data_store: data_store.as_ref().clone(),
            templates: templates.as_ref().clone(),
        },
        &req,
        stream,
    )
}

#[get("/ws/lines/{line_ref}")]
async fn line_websocket(
    req: HttpRequest,
    line_ref: web::Path<String>,
    stream: web::Payload,
    central: web::Data<Addr<CentralDispatch>>,
    data_store: web::Data<Addr<DataStore>>,
    templates: web::Data<Addr<Templates>>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("new websocket watching {line_ref}");

    ws::start(
        SessionActor {
            central: central.as_ref().clone(),
            watching: Watching::Line(line_ref.clone()),
            data_store: data_store.as_ref().clone(),
            templates: templates.as_ref().clone(),
        },
        &req,
        stream,
    )
}

#[get("/")]
async fn index(templates: web::Data<Addr<Templates>>) -> impl Responder {
    let s = templates
        .send(RenderTemplate {
            template: "index.html",
            context: tera::Context::new(),
        })
        .await
        .unwrap()
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[get("/lines/{id}")]
async fn line(
    line_ref: web::Path<String>,
    templates: web::Data<Addr<Templates>>,
) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("line_ref", &line_ref.as_str());
    let s = templates
        .send(RenderTemplate {
            template: "line_index.html",
            context,
        })
        .await
        .unwrap()
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
