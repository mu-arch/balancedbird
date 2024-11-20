use std::collections::HashMap;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::{Client, Error};
use reqwest::header::HeaderMap;
use serde_json::{from_str, json};
use scraper::{Html, Selector};
use crate::types::{MetarData, MetarDataReturned};

// Handler function for the /weather/:code route
pub async fn weather_handler(Path(code): Path<String>) -> Result<Json<MetarData>, (StatusCode, &'static str)> {
    if let Some(metar) = fetch_metar_data(&code).await {
        
        let metarData = MetarDataReturned { 
            metar_id: val, 
            icao_id: val, 
            receipt_time: val, 
            temp: val, 
            dewp: val, 
            wdir: val, 
            wspd: val, 
            altimeter: val, 
            slp: val, 
            raw_ob: val, 
            name: val };
        
        Ok(Json(metar))
    } else {
        Err((StatusCode::BAD_GATEWAY, "Failed to fetch METAR data"))
    }
}


async fn fetch_metar_data(code: &str) -> Option<MetarData> {
    let url = format!(
        "https://aviationweather.gov/api/data/metar?ids={}&format=json",
        code
    );

    // Send the GET request and parse the first item in the array
    Client::new()
        .get(&url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
        .and_then(|body| {
            let mut data: Vec<MetarData> = from_str(&body).ok()?;
            data.pop()
        })
}


#[derive(Debug)]
struct AirportInfo {
    field_elevation: String,
    runways: Vec<String>,
    airport_diagram_link: String,
}

/*
fn extract_airport_info(html_content: &str) -> Result<AirportInfo, Box<dyn Error>> {
    // Parse the HTML content
    let document = Html::parse_document(html_content);

    // Selectors for the required data
    let elevation_selector = Selector::parse("h3:contains('Location') + table tr").unwrap();
    let runway_selector = Selector::parse("h3:contains('Runway Information') + h4, h3:contains('Runway Information') + h4 + table tr").unwrap();
    let diagram_selector = Selector::parse("img[alt^='Diagram of']").unwrap();

    // Extract field elevation
    let mut field_elevation = String::from("N/A");
    for element in document.select(&elevation_selector) {
        if let Some(header) = element.select(&Selector::parse("td:nth-child(1)").unwrap()).next() {
            if header.inner_html().contains("Elevation:") {
                if let Some(value) = element.select(&Selector::parse("td:nth-child(2)").unwrap()).next() {
                    field_elevation = value.inner_html().trim().to_string();
                    break;
                }
            }
        }
    }

    // Extract runway information
    let mut runways = Vec::new();
    let mut current_runway = String::new();
    let mut collecting = false;
    for element in document.select(&runway_selector) {
        if element.value().name() == "h4" {
            // Found a new runway header
            current_runway = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            collecting = true;
        } else if collecting {
            // Collect runway details
            let details = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            runways.push(format!("{}: {}", current_runway, details));
        }
    }

    // Extract airport diagram link
    let airport_diagram_link = if let Some(img_element) = document.select(&diagram_selector).next() {
        if let Some(src) = img_element.value().attr("src") {
            src.to_string()
        } else {
            String::from("N/A")
        }
    } else {
        String::from("N/A")
    };

    Ok(AirportInfo {
        field_elevation,
        runways,
        airport_diagram_link,
    })
}


 */