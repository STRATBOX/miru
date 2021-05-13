use actix_web::{web, Responder};
use chrono::Utc;
use tracing::instrument;
use ulid::Ulid;

use crate::domain::ping::Ping;

#[instrument]
pub async fn ping() -> impl Responder {
    web::Json(Ping {
        id: Ulid::new().to_string().to_lowercase(),
        msg: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        ts: Utc::now().timestamp_millis(),
    })
}
