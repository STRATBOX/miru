use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer, Responder};
use chrono::Utc;
use dotenv::dotenv;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::{env, io};
use ulid::Ulid;

#[derive(Debug, Deserialize, Serialize)]
struct Ping {
    id: String,
    msg: String,
    ts: i64,
}

async fn index() -> impl Responder {
    let msg = env::var("SERVICE").expect("SERVICE not set");
    let p = Ping {
        id: Ulid::new().to_string().to_lowercase(),
        msg,
        ts: Utc::now().timestamp_millis(),
    };

    web::Json(p)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST not set");
    let port = env::var("PORT").expect("PORT not set");

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", host, port))?,
    };

    server.run().await
}
