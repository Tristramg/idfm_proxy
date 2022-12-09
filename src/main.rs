use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use actix::prelude::*;
use actix_web::{middleware, web, App, HttpServer};
use color_eyre::eyre::format_err;
use objects::{LineReference, StopReference};
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

mod actors;
mod messages;
mod objects;
mod routes;
mod status;
mod templates;
use actors::*;
use routes::*;

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

fn parse_stop_referential() -> color_eyre::Result<HashMap<String, StopReference>> {
    let ref_file = std::fs::File::open("static/data/arrets.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(ref_file);
    let mut stop_referential = HashMap::new();
    for result in rdr.deserialize() {
        let record: StopReference = result?;
        stop_referential.insert(
            format!("STIF:StopPoint:Q:{}:", record.id).to_string(),
            record,
        );
    }
    tracing::info!(
        "Parsed stop referential with {} stops",
        stop_referential.len()
    );
    Ok(stop_referential)
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    setup_logger();

    let dispatch_addr = CentralDispatch {
        sessions: Vec::new(),
    }
    .start();

    let old_data = std::fs::read_to_string("static/data/idfm_estimated_timetable.latest.json")
        .map_err(|_| format_err!("casting err"))
        .and_then(|json| {
            serde_json::from_str::<siri_lite::siri::SiriResponse>(&json)
                .map_err(|_| format_err!("casting err"))
        });
    let data_store = DataStore {
        central_dispatch: dispatch_addr.clone(),
        pt_data: None,
        line_referential: Arc::new(parse_line_referential()?),
        stop_referential: Arc::new(parse_stop_referential()?),
    }
    .start();

    if let Ok(old_data) = old_data {
        let vjs = old_data
            .siri
            .service_delivery
            .ok_or(format_err!("Siri: could not find service_delivery"))?
            .estimated_timetable_delivery
            .into_iter()
            .flat_map(|delivery| {
                delivery
                    .estimated_journey_version_frame
                    .into_iter()
                    .flat_map(|frame| frame.estimated_vehicle_journey)
            })
            .collect();
        data_store.do_send(messages::SiriUpdate { vjs });
        tracing::info!("Re-using old data");
    }

    let _siri_fetcher = SiriFetcher {
        apikey: std::env::var("API_KEY")
            .as_deref()
            .expect("Missing API_KEY environment variable")
            .to_string(),
        uri: "https://prim.iledefrance-mobilites.fr/marketplace/estimated-timetable".to_string(),
        data_store: data_store.clone(),
    }
    .start();

    let _gtfs_fetch = actors::GtfsFetcher {
        dispatch: data_store.clone(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(dispatch_addr.clone()))
            .app_data(web::Data::new(data_store.clone()))
            .service(actix_files::Files::new("/static", "./static"))
            .service(index)
            .service(line)
            .service(status::status)
            .service(websocket)
            .service(line_websocket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| format_err! {"Could not start the server: {e}"})?;
    Ok(())
}
