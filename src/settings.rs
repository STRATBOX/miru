use color_eyre::Result;
use config::{Config, Environment};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: i32,
    pub service: String,
}

impl Settings {
    #[instrument]
    pub fn from_env() -> Result<Settings> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = Config::new();
        c.merge(Environment::default())?;
        c.try_into().context("loading config from environment")
    }
}
