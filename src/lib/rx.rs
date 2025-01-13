use reqwest::{Client, Error as ReqwestError};
use serde_json::{from_str, json};
use crate::types::{MetarDataRaw, WeatherDataReturned, Runway, WindDirection};
use tracing::error;

pub async fn fetch_metar_data(code: &str) -> Option<MetarDataRaw> {
    let url = format!(
        "https://aviationweather.gov/api/data/metar?ids={}&format=json",
        code
    );

    let client = Client::new();

    // Send the GET request and get the text of the response
    let text = client
        .get(&url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    // Parse the JSON. The response is an array of 1 (or more) items, so we
    // deserialize into a Vec, then just get the first (or last).
    let mut data: Vec<MetarDataRaw> = serde_json::from_str(&text).ok()?;
    data.pop()
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