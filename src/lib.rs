// dependencies
use actix_web::dev::Server;
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

// module definitions
mod api;
mod models;
mod settings;

// use module dependencies
use crate::api::ping;
use crate::settings::Settings;

pub fn get_config() -> Settings {
    Settings::from_env().expect("Server configuration error")
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(ping))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
