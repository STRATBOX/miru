use actix_web::{web, Responder};
use chrono::Utc;
use tracing::instrument;
use ulid::Ulid;

use crate::models::Ping;

#[instrument]
pub async fn ping() -> impl Responder {
    web::Json(Ping {
        id: Ulid::new().to_string().to_lowercase(),
        msg: String::from("miru"),
        ts: Utc::now().timestamp_millis(),
    })
}