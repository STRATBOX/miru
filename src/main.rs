// dependencies
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;
use listenfd::ListenFd;

// module definitions
mod config;
mod api;
mod models;

// use module dependencies
use crate::config::Config;
use crate::api::index;

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
