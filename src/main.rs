// dependencies
use color_eyre::Result;
use std::net::TcpListener;

// module definitions

// use module dependencies
use miru::configuration::get_configuration;
use miru::startup::run;
use miru::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<()> {
    // let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let app_name = env!("CARGO_PKG_NAME").to_string();

    // setup tracing subscriber
    let subscriber = get_subscriber(app_name, "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Get app config settings
    let config = get_configuration();

    // bind TCP listener to specified socket address
    let listener = TcpListener::bind(format!("{}:{}", config.app.host, config.app.port))
        .expect("Failed to bind random port");

    // Run server
    run(listener)?.await?;

    Ok(())
}
