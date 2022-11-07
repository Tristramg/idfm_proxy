use crate::{central_dispatch::CentralDispatch, messages::UpdateVJs};
use actix::prelude::*;
use color_eyre::eyre::{eyre, Result};
use siri_lite::{service_delivery::EstimatedVehicleJourney, siri::SiriResponse};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct FetchSiri;

#[derive(Clone)]
pub struct SiriFetcher {
    pub dispatch: Addr<CentralDispatch>,
    pub vehicle_journeys: Arc<Vec<EstimatedVehicleJourney>>,
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
                Ok(vjs) => act.dispatch.do_send(UpdateVJs { vjs: Arc::new(vjs) }),
                Err(e) => tracing::info!(" {e}"),
            })
            .wait(ctx);
    }
}

async fn fetch(uri: String, apikey: String) -> Result<Vec<EstimatedVehicleJourney>> {
    tracing::info!("Starting fetching");
    let response = reqwest::Client::new()
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
    let estimated_vehicle_journeys = serde_json::from_str::<SiriResponse>(&response)
        .map_err(|err| eyre!("Siri: could not parse json: {err}"))?
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
        });

    Ok(estimated_vehicle_journeys.collect())
}
