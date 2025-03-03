use std::error::Error;

use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde_json::Value;
use tracing::{error, info};

use crate::Config;

pub struct GraphQLService {
    client: Client,
    base_url: String,
    token: String,
}

impl GraphQLService {
    pub fn new(config: &Config) -> Self {
        GraphQLService {
            client: Client::new(),
            base_url: config.api_url.clone(),
            token: config.api_token.clone(),
        }
    }

    pub async fn execute(&self, query: Value) -> Result<Value, Box<dyn Error>> {
        info!("Sending GraphQL request to {}", self.base_url);
        let response = self
            .client
            .post(&self.base_url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .json(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            error!("GraphQL request failed: {}", response.status());
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        let json: Value = response.json().await?;
        Ok(json)
    }
}
