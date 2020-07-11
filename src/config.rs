use color_eyre::Result;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub service: String
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;
        c.try_into()
            .context("loading config from environment")
    }
}