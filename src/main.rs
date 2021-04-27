// dependencies
use color_eyre::Result;
use dotenv::dotenv;
use std::net::TcpListener;

// module definitions

// use module dependencies
use miru::run;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Get app config settings
    let config = miru::get_config();

    // bind TCP listener to specified socket address
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .expect("Failed to bind random port");

    // Run server
    run(listener)?.await?;

    Ok(())
}
