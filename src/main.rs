use std::str::FromStr;

use actix::prelude::*;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use askama_actix::Template;
use color_eyre::{eyre::format_err, Result};
use idfm_proxy::central_dispatch::CentralDispatch;
use idfm_proxy::session_actor::SessionActor;
use idfm_proxy::siri_stuff::SiriFetcher;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

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
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    let dispatch_addr = CentralDispatch {
        sessions: Vec::new(),
        vjs: None,
    }
    .start();

    let _siri_fetcher = SiriFetcher {
        apikey: "".to_string(),
        uri: "https://prim.iledefrance-mobilites.fr/marketplace/estimated-timetable".to_string(),
        vehicle_journeys: std::sync::Arc::new(Vec::new()),
        dispatch: dispatch_addr.clone(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dispatch_addr.clone()))
            .service(index)
            .service(websocket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| format_err! {"Could not start the server: {e}"})?;
    Ok(())
}
