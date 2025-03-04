use crate::{config::Config, models::stop::Stop};
use futures::future::join_all;
use reqwest::{Client, StatusCode};
use secrecy::ExposeSecret;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct GeocodingService<'a> {
    client: Client,
    config: &'a Config,
}

impl<'a> GeocodingService<'a> {
    pub fn new(client: Client, config: &'a Config) -> Self {
        GeocodingService { client, config }
    }

    pub async fn geocode_address(&self, stop: &mut Stop) -> Result<(), Box<dyn Error>> {
        let base_url = format!(
            "{}/search/geocode/v6/forward",
            self.config.map_box_client_setting.base_url
        );
        let url = reqwest::Url::parse_with_params(
            &base_url,
            &[
                ("q", stop.position.as_str()),
                (
                    "access_token",
                    self.config.map_box_client_setting.map_api_token.expose_secret(),
                ),
            ],
        )?;

        let max_retries = 3;
        let mut attempt = 0;
        let mut delay = Duration::from_secs(1); // Start with 1s delay

        loop {
            let response_result = self.client.get(url.clone()).send().await;
            match response_result {
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::TOO_MANY_REQUESTS => {
                            if attempt >= max_retries {
                                error!(
                                    "Max retries ({}) reached for {}. Rate limit exceeded.",
                                    max_retries, url
                                );
                                return Err("Rate limit exceeded after retries".into());
                            }

                            let retry_after = resp
                                .headers()
                                .get("Retry-After")
                                .and_then(|v| v.to_str().ok())
                                .and_then(|s| s.parse::<u64>().ok())
                                .unwrap_or(delay.as_secs());

                            warn!(
                                "Rate limit hit for {}. Retrying after {}s (attempt {}/{})",
                                url, retry_after, attempt + 1, max_retries
                            );
                            tokio::time::sleep(Duration::from_secs(retry_after)).await;
                            attempt += 1;
                            delay *= 2; // Exponential backoff
                            continue;
                        }
                        StatusCode::OK => {
                            let json_result = resp.json::<Value>().await;
                            let json = match json_result {
                                Ok(value) => value,
                                Err(e) => {
                                    error!("Failed to parse JSON from {}: {:?}", url, e);
                                    return Err(e.into());
                                }
                            };

                            let features = json["features"]
                                .as_array()
                                .ok_or("No features in Mapbox response")?;
                            let feature =
                                features.first().ok_or("No results found for position")?;

                            stop.position = feature["properties"]["full_address"]
                                .as_str()
                                .ok_or("Missing full_address")?
                                .to_string();

                            let coords = feature["geometry"]["coordinates"]
                                .as_array()
                                .ok_or("Missing geometry coordinates")?;
                            stop.longitude = coords[0]
                                .as_f64()
                                .ok_or("Invalid longitude")?
                                .to_string();
                            stop.latitude = coords[1]
                                .as_f64()
                                .ok_or("Invalid latitude")?
                                .to_string();

                            info!(
                                "Geocoded {} to ({}, {})",
                                stop.id, stop.latitude, stop.longitude
                            );
                            return Ok(());
                        }
                        other => {
                            error!(
                                "Unexpected status code {} for {}. Response: {:?}",
                                other,
                                url,
                                resp.text().await
                            );
                            return Err(format!("Unexpected status code: {}", other).into());
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to send request to {}: {:?}", url, e);
                    return Err(e.into());
                }
            }
        }
    }

    pub async fn geocode_stops(&self, stops: &mut [Stop]) -> Result<(), Box<dyn Error>> {
        const REQUESTS_PER_SECOND: usize = 10; // Mapbox free tier limit
        const BATCH_SIZE: usize = REQUESTS_PER_SECOND; // 10 requests per batch
        const BATCH_DELAY: Duration = Duration::from_secs(1); // 1s between batches

        for chunk in stops.chunks_mut(BATCH_SIZE) {
            let chunk_tasks: Vec<_> = chunk
                .iter_mut()
                .map(|stop| {
                    let mut stop_clone = stop.clone();
                    async move {
                        let result = self.geocode_address(&mut stop_clone).await;
                        (stop_clone, result)
                    }
                })
                .collect();

            let results = join_all(chunk_tasks).await;
            for (i, (updated_stop, result)) in results.into_iter().enumerate() {
                match result {
                    Ok(()) => chunk[i] = updated_stop,
                    Err(e) => error!("Failed to geocode stop {}: {}", chunk[i].id, e),
                }
            }

            if chunk.len() == BATCH_SIZE {
                // Only sleep if we processed a full batch, to avoid unnecessary delay at the end
                info!(
                    "Processed batch of {} stops. Waiting {}ms before next batch.",
                    BATCH_SIZE,
                    BATCH_DELAY.as_millis()
                );
                tokio::time::sleep(BATCH_DELAY).await;
            }
        }

        info!("Geocoded {} stops successfully", stops.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BackendApiSetting, Config, MapBoxClientSetting};
    use mockito::{Mock, Server};

    async fn setup_mock_server(server: &mut Server) -> (Mock, Config) {
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "features": [
                        {
                            "geometry": {
                                "type": "Point",
                                "coordinates": [-74.103439, 4.605241]
                            },
                            "properties": {
                                "full_address": "Bogotá, 111611, Colombia"
                            }
                        }
                    ]
                }"#,
            )
            .create();

        let config = Config {
            backend_api_setting: BackendApiSetting {
                base_url: "http://example.com".to_string(),
                api_token: "test_token".into(),
            },
            map_box_client_setting: MapBoxClientSetting {
                base_url: server.url(),
                map_api_token: "test_token".into(),
            },
        };

        (mock, config)
    }

    async fn setup_mock_server_no_results(server: &mut Server) -> (Mock, Config) {
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"features": []}"#)
            .create();

        let config = Config {
            backend_api_setting: BackendApiSetting {
                base_url: "http://example.com".to_string(),
                api_token: "test_token".into(),
            },
            map_box_client_setting: MapBoxClientSetting {
                base_url: server.url(),
                map_api_token: "test_token".into(),
            },
        };

        (mock, config)
    }

    #[tokio::test]
    async fn test_geocode_address() {
        let mut server = Server::new_async().await;
        let (mock, config) = setup_mock_server(&mut server).await;
        let client = Client::new();
        let service = GeocodingService::new(client ,&config);

        let mut stop = Stop {
            id: "1".to_string(),
            position: "111611".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            stop_id: "TS00011".to_string(),
        };

        let result = service.geocode_address(&mut stop).await;

        assert!(result.is_ok());
        assert_eq!(stop.position, "Bogotá, 111611, Colombia");
        mock.assert();
    }

    #[tokio::test]
    async fn test_geocode_address_no_results() {
        let mut server = Server::new_async().await;
        let (mock, config) = setup_mock_server_no_results(&mut server).await;
        let client = Client::new();
        let service = GeocodingService::new(client ,&config);

        let mut stop = Stop {
            id: "1".to_string(),
            position: "Unknown".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            stop_id: "TS00011".to_string(),
        };

        let result = service.geocode_address(&mut stop).await;

        assert!(result.is_err());
        mock.assert();
    }
}
