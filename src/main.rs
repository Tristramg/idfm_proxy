use actix::prelude::*;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use anyhow::Result;
use askama_actix::Template;
use idfm_proxy::central_dispatch::CentralDispatch;
use idfm_proxy::session_actor::SessionActor;

#[get("/ws")]
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    central: web::Data<Addr<CentralDispatch>>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("new websocket");

    ws::start(
        SessionActor {
            central: central.as_ref().clone(),
        },
        &req,
        stream,
    )
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

#[get("/")]
async fn index() -> impl Responder {
    HelloTemplate {}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let dispatch = ;
    let dispatch_addr = CentralDispatch::start(CentralDispatch {
        sessions: Vec::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dispatch_addr.clone()))
            .service(index)
            .service(websocket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
