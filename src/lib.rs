// dependencies
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

// module definitions
mod api;
mod models;
mod settings;

// use module dependencies
use crate::api::ping;
use crate::settings::Settings;

pub fn run() -> Result<Server, std::io::Error> {
    let config = Settings::from_env().expect("Server configuration error");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(ping))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run();

    Ok(server)
}
