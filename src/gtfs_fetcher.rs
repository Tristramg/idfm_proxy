use crate::central_dispatch::CentralDispatch;
use actix::prelude::*;
use color_eyre::eyre::Result;
use gtfs_structures::Gtfs;

#[derive(Message)]
#[rtype(result = "()")]
pub struct FetchSiri;

#[derive(Clone)]
pub struct GtfsFetcher {
    pub dispatch: Addr<CentralDispatch>,
}

impl Actor for GtfsFetcher {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("Starting the gtfs fetcher!");
        self.update_gtfs(ctx);

        ctx.run_interval(std::time::Duration::from_secs(3600), |act, ctx| {
            act.update_gtfs(ctx)
        });
    }
}

impl GtfsFetcher {
    fn update_gtfs(&mut self, ctx: &mut Context<Self>) {
        fetch()
            .into_actor(self)
            .map(|r, _act, _ctx| match r {
                Ok(gtfs) => tracing::info!("Got gtfs with {} stops", gtfs.stops.len()),
                Err(e) => tracing::info!(" {e}"),
            })
            .wait(ctx);
    }
}

async fn fetch() -> Result<Gtfs> {
    tracing::info!("Starting downloading GTFS");
    let filename = format!(
        "static/data/IDFM_gtfs.{}.zip",
        chrono::offset::Utc::now().to_rfc3339()
    );
    let mut file = std::fs::File::create(&filename)?;

    let bytes = reqwest::get("https://data.iledefrance-mobilites.fr/api/v2/catalog/datasets/offre-horaires-tc-gtfs-idfm/files/a925e164271e4bca93433756d6a340d1")
    .await?
    .bytes().await?;

    std::io::copy(&mut std::io::Cursor::new(bytes), &mut file)?;

    tracing::info!("Got the GTFS, starting parsing");
    let gtfs = gtfs_structures::GtfsReader::default()
        .read_stop_times(false)
        .trim_fields(false)
        .read_from_path(&filename)?;

    std::fs::rename(filename, "static/data/IDFM_gtfs.zip")?;
    Ok(gtfs)
}
