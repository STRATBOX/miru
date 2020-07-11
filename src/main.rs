mod config;

use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer, Responder};

use color_eyre::Result;

use chrono::Utc;

use dotenv::dotenv;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

// use module dependencies
use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
struct Ping {
    id: String,
    msg: String,
    ts: i64,
}

async fn index() -> impl Responder {
    let p = Ping {
        id: Ulid::new().to_string().to_lowercase(),
        msg: String::from("miru"),
        ts: Utc::now().timestamp_millis(),
    };

    web::Json(p)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    
    
    let config = Config::from_env()
            .expect("Server configuration error");

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", config.host, config.port))?,
    };

    server.run().await?;

    Ok(())
}
