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
    
    // setup tracing subscriber
    let subscriber = get_subscriber("miru".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Get app config settings
    let config = get_configuration();

    // bind TCP listener to specified socket address
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .expect("Failed to bind random port");

    // Run server
    run(listener)?.await?;

    Ok(())
}
