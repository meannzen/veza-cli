use secrecy::SecretString;
use std::env;
use tracing::{info, warn};

#[derive(Debug)]
pub struct Config {
    pub backend_api_setting: BackendApiSetting,
    pub map_box_client_setting: MapBoxClientSetting,
}

#[derive(Debug)]
pub struct MapBoxClientSetting {
    pub base_url: String,
    pub map_api_token: SecretString,
}

#[derive(Debug)]
pub struct BackendApiSetting {
    pub base_url: String,
    pub api_token: SecretString,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if present, log if it fails
        if let Err(e) = dotenv::dotenv() {
            warn!("Failed to load .env file: {}", e);
        }

        let api_url = env::var("API_URL").map_err(|e| ConfigError::MissingEnvVar("API_URL", e))?;
        let api_token =
            env::var("API_TOKEN").map_err(|e| ConfigError::MissingEnvVar("API_TOKEN", e))?;
        let map_box_url = env::var("MAP_BOX_URL")
            .or_else(|_| Ok("https://api.mapbox.com".to_string())) // Default value
            .map_err(|e| ConfigError::MissingEnvVar("MAP_BOX_URL", e))?;
        let map_box_token = env::var("MAP_BOX_TOKEN")
            .map_err(|e| ConfigError::MissingEnvVar("MAP_BOX_TOKEN", e))?;

        // Basic validation
        if api_url.trim().is_empty() {
            return Err(ConfigError::InvalidValue("API_URL", "URL cannot be empty"));
        }
        if map_box_url.trim().is_empty() {
            return Err(ConfigError::InvalidValue(
                "MAP_BOX_URL",
                "URL cannot be empty",
            ));
        }

        info!("Loaded backend API base URL: {}", api_url);
        info!("Loaded MapBox base URL: {}", map_box_url);

        Ok(Config {
            backend_api_setting: BackendApiSetting {
                base_url: api_url,
                api_token: api_token.into(),
            },
            map_box_client_setting: MapBoxClientSetting {
                base_url: map_box_url,
                map_api_token: map_box_token.into(),
            },
        })
    }
}

/// Custom error type for configuration loading issues.
#[derive(Debug)]
pub enum ConfigError {
    MissingEnvVar(&'static str, env::VarError),
    InvalidValue(&'static str, &'static str),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingEnvVar(key, e) => {
                write!(f, "Missing environment variable {}: {}", key, e)
            }
            ConfigError::InvalidValue(key, msg) => write!(f, "Invalid value for {}: {}", key, msg),
        }
    }
}

impl std::error::Error for ConfigError {}
