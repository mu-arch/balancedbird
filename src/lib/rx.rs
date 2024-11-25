use reqwest::{Client, Error as ReqwestError};
use serde_json::{from_str, json};
use crate::types::{MetarDataRaw, MetarDataReturned, Runway, WindDirection};
use tracing::error;

pub(crate) async fn fetch_metar_data(code: &str) -> Option<MetarDataRaw> {
    let url = format!(
        "https://aviationweather.gov/api/data/metar?ids={}&format=json",
        code
    );

    // Create a new HTTP client
    let client = Client::new();

    // Send the GET request
    let response = client.get(&url).send().await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                error!("Request failed with status: {}", resp.status());
                return None;
            }

            // Extract the response text
            let body = resp.text().await;

            match body {
                Ok(text) => {
                    // Parse the JSON response
                    let data: Result<Vec<MetarDataRaw>, _> = serde_json::from_str(&text);

                    match data {
                        Ok(mut metar_data) => {
                            if let Some(last_data) = metar_data.pop() {
                                // Successfully parsed METAR data
                                Some(last_data)
                            } else {
                                error!("Parsed METAR data is empty for code: {}", code);
                                None
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse JSON for code {}: {}", code, e);
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read response body for code {}: {}", code, e);
                    None
                }
            }
        }
        Err(e) => {
            error!("HTTP request failed for code {}: {}", code, e);
            None
        }
    }
}

pub(crate) async fn fetch_html(url: String) -> Option<String> {
    // Send the GET request and return the HTML content
    Client::new()
        .get(url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
}