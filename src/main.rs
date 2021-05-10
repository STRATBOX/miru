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
    // Get app config settings
    let config = get_configuration();

    // setup tracing subscriber
    let subscriber = get_subscriber("miru".into(), "info".into());
    init_subscriber(subscriber);

    // bind TCP listener to specified socket address
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .expect("Failed to bind random port");

    // Run server
    run(listener)?.await?;

    Ok(())
}
