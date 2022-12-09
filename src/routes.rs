use crate::actors::*;
use crate::templates;
use actix::prelude::*;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use color_eyre::Result;

#[get("/ws")]
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    central: web::Data<Addr<CentralDispatch>>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("new websocket");

    ws::start(
        SessionActor {
            central: central.as_ref().clone(),
            watching: Watching::Index,
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
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("new websocket watching {line_ref}");

    ws::start(
        SessionActor {
            central: central.as_ref().clone(),
            watching: Watching::Line(line_ref.clone()),
        },
        &req,
        stream,
    )
}

#[get("/")]
async fn index() -> impl Responder {
    let s = templates::TEMPLATES
        .render("index.html", &tera::Context::new())
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[get("/lines/{id}")]
async fn line(line_ref: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("line_ref", &line_ref.as_str());
    let s = templates::TEMPLATES
        .render("line_index.html", &context)
        .unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
