use color_eyre::Result;
use config::Config;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
// use tracing_subscriber::EnvFilter;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub app: AppSettings
}
#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: i32,
    pub name: String,
}

impl Configuration {
    #[instrument]
    pub fn from_env() -> Result<Configuration> {
        // dotenv().ok();

        info!("Loading configuration");

        // initialise configuration reader
        let mut c = Config::default();

        // Add configuration values from a file named `app`.
        // This will look for any top-level file with an extension
        // that `config` knows how to parse: yaml, json, etc.
        c.merge(config::File::with_name("app"))?;

        // Read config from .env variable
        // c.merge(Environment::default())?;
        c.try_into().context("loading config from environment")
    }
}

pub fn get_configuration() -> Configuration {
    Configuration::from_env().expect("Server configuration error")
}
