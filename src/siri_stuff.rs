use crate::{central_dispatch::CentralDispatch, messages::SiriUpdate};
use actix::prelude::*;
use color_eyre::eyre::{eyre, ErrReport, Result};
use siri_lite::{service_delivery::EstimatedVehicleJourney, siri::SiriResponse};
use std::io::Write;

#[derive(Message)]
#[rtype(result = "()")]
pub struct FetchSiri;

#[derive(Clone)]
pub struct SiriFetcher {
    pub dispatch: Addr<CentralDispatch>,
    pub uri: String,
    pub apikey: String,
}

impl Actor for SiriFetcher {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("Starting the siri fetcher!");
        self.update_vjs(ctx);

        ctx.run_interval(std::time::Duration::from_secs(90), |act, ctx| {
            act.update_vjs(ctx)
        });
    }
}
impl SiriFetcher {
    fn update_vjs(&mut self, ctx: &mut Context<Self>) {
        let u = self.uri.clone();
        let k = self.apikey.clone();

        fetch(u, k)
            .into_actor(self)
            .map(|r, act, _ctx| match r {
                Ok(data) => act.dispatch.do_send(SiriUpdate { vjs: data }),
                Err(e) => tracing::info!(" {e}"),
            })
            .wait(ctx);
    }
}

async fn fetch(uri: String, apikey: String) -> Result<Vec<EstimatedVehicleJourney>> {
    tracing::info!("Starting fetching");
    let response = reqwest::Client::builder()
        .gzip(true)
        .build()?
        .get(uri)
        .header("apikey", apikey)
        .query(&[("LineRef", "ALL")])
        .send()
        .await
        .map_err(|err| eyre!("Siri request: could execute the query: {err}"))?
        .text()
        .await
        .map_err(|err| eyre!("Siri: could not extract request body: {err} "))?;

    tracing::info!("Got the timetable, starting parsing");

    let vjs = serde_json::from_str::<SiriResponse>(&response)
        .map_err(|err| handle_unparsable(err, &response))?
        .siri
        .service_delivery
        .ok_or(eyre!("Siri: could not find service_delivery"))?
        .estimated_timetable_delivery
        .into_iter()
        .flat_map(|delivery| {
            delivery
                .estimated_journey_version_frame
                .into_iter()
                .flat_map(|frame| frame.estimated_vehicle_journey)
        })
        .collect();

    save(&response)?;
    Ok(vjs)
}

fn handle_unparsable(err: serde_json::Error, response: &str) -> ErrReport {
    let filename = format!(
        "siri_estimated_timetable_{}.json",
        chrono::offset::Utc::now().to_rfc3339()
    );
    let mut file = std::fs::File::create(&filename)
        .map_err(|err| eyre!("Could not create file for failed siri: {err}"))
        .unwrap();
    file.write(response.as_bytes())
        .map_err(|err| eyre!("Could not write failed siri to disk: {err}"))
        .unwrap();
    eyre!("Siri: could not parse json: {err}, see file in {filename}")
}

fn save(response: &str) -> Result<()> {
    let filename = format!(
        "static/data/siri_estimated_timetable_{}.json",
        chrono::offset::Utc::now().to_rfc3339()
    );
    let mut file = std::fs::File::create(&filename)?;
    file.write(response.as_bytes())?;
    std::fs::rename(filename, "static/data/idfm_estimated_timetable.latest.json")?;
    Ok(())
}
