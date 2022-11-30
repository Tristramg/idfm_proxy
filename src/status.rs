use crate::{central_dispatch::CentralDispatch, messages::StatusDemand};
use actix_web::{web, get};
use actix::Addr;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Status {
	pub nb_open_connections: usize,
	//last_rt_loaded: NaiveDateTime
}

#[get("/status")]
async fn status(central: web::Data<Addr<CentralDispatch>>) -> Result<web::Json<Status>, actix_web::Error> {
	let s = central.send(StatusDemand{}).await.map_err(|e| {
		tracing::error!("impossible to query central dispatch, error : {}", e);
		actix_web::error::ErrorInternalServerError(e)
	})?;
    Ok(web::Json(s.as_ref().clone()))
}
