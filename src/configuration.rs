use color_eyre::Result;
use config::{Config, Environment};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
// use tracing_subscriber::EnvFilter;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub host: String,
    pub port: i32,
    pub service: String,
}

impl Configuration {
    #[instrument]
    pub fn from_env() -> Result<Configuration> {
        dotenv().ok();

        info!("Loading configuration");

        let mut c = Config::new();
        c.merge(Environment::default())?;
        c.try_into().context("loading config from environment")
    }
}

pub fn get_configuration() -> Configuration {
    Configuration::from_env().expect("Server configuration error")
}
