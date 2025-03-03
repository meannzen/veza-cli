use std::env;

use tracing::info;

pub struct Config {
    pub api_url: String,
    pub api_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        let api_url = env::var("API_URL")?;
        let api_token = env::var("API_TOKEN")?;

        info!("Loaded  backend URL: {}", api_url);
        info!("Loadde backned token: [redacted]");

        Ok(Self { api_url, api_token })
    }
}
