use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use actix::prelude::*;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use color_eyre::{eyre::format_err, Result};
use idfm_proxy::central_dispatch::CentralDispatch;
use idfm_proxy::objects::LineReference;
use idfm_proxy::session_actor::{SessionActor, Watching};
use idfm_proxy::siri_stuff::SiriFetcher;
use idfm_proxy::status::status;
use idfm_proxy::templates;
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
    templates::Index {}
}



#[get("/lines/{id}")]
async fn line(line_ref: web::Path<String>) -> impl Responder {
    templates::LineIndex {
        line_ref: line_ref.to_string(),
    }
}

fn setup_logger() {
    color_eyre::install().unwrap();

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();
}

fn parse_line_referential() -> color_eyre::Result<HashMap<String, LineReference>> {
    let ref_file = std::fs::File::open("static/data/referentiel-des-lignes.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(ref_file);
    let mut line_referential = HashMap::new();
    for result in rdr.deserialize() {
        let record: LineReference = result?;
        line_referential.insert(format!("STIF:Line::{}:", record.id).to_string(), record);
    }
    tracing::info!(
        "Parsed line referential with {} lines",
        line_referential.len()
    );
    Ok(line_referential)
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    setup_logger();

    let dispatch_addr = CentralDispatch {
        sessions: Vec::new(),
        pt_data: None,
        line_referential: Arc::new(parse_line_referential()?),
    }
    .start();

    let _siri_fetcher = SiriFetcher {
        apikey: std::env::var("API_KEY")
            .as_deref()
            .expect("Missing API_KEY environment variable")
            .to_string(),
        uri: "https://prim.iledefrance-mobilites.fr/marketplace/estimated-timetable".to_string(),
        dispatch: dispatch_addr.clone(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(dispatch_addr.clone()))
            .service(actix_files::Files::new("/static", "./static"))
            .service(index)
            .service(line)
            .service(status)
            .service(websocket)
            .service(line_websocket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| format_err! {"Could not start the server: {e}"})?;
    Ok(())
}
