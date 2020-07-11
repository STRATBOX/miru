use color_eyre::Result;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub service: String
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;
        c.try_into()
            .context("loading config from environment")
    }
}