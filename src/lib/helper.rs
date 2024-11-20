use std::cmp::Ordering;
use std::error::Error;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::{Client, Error as ReqwestError};
use serde_json::{from_str, json};
use crate::types::{MetarDataRaw, MetarDataReturned};

// Handler function for the /weather/:code route
pub async fn weather_handler(Path(code): Path<String>) -> Result<(StatusCode, Json<MetarDataReturned>), (StatusCode, &'static str)> {
    // Fetch airport data
    let airnav_page_data = fetch_html(format!("https://www.airnav.com/airport/{}", &code))
        .await
        .ok_or_else(|| (StatusCode::BAD_GATEWAY, "Network error"))?;

    // Extract airport information
    let extracted_airport_data = extract_airport_info(&airnav_page_data)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Airport not found"))?;

    // Debugging (consider using proper logging in production)
    dbg!(&extracted_airport_data);

    // Fetch METAR data
    let metar = fetch_metar_data(&code).await
        .ok_or_else(|| (StatusCode::BAD_GATEWAY, "Failed to fetch METAR data"))?;

    // Perform calculations
    let altimeter_inhg = hpa_to_inhg(metar.altim);
    let density_altitude = calculate_density_altitude(
        metar.temp,
        extracted_airport_data.field_elevation,
        metar.altim,
    );

    // Determine the best runway based on crosswind
    let (best_runway, min_crosswind, best_headwind) = extracted_airport_data.runways.iter()
        .map(|runway| {
            let crosswind = calculate_crosswind(metar.wdir as f64, metar.wspd as f64, runway.heading_magnetic);
            let headwind = calculate_headwind(metar.wdir as f64, metar.wspd as f64, runway.heading_magnetic);
            (runway.number.clone(), crosswind, headwind)
        })
        .min_by(|a, b| {
            a.1.abs()
                .partial_cmp(&b.1.abs())
                .unwrap_or(Ordering::Equal)
        })
        .unwrap_or(("Unknown".to_string(), 0.0, 0.0));

    // Prepare the response data
    let metar_data_returned = MetarDataReturned {
        metar_id: metar.metar_id,
        temp: metar.temp,
        dewp: metar.dewp,
        wdir: metar.wdir,
        wspd: metar.wspd,
        altimeter: altimeter_inhg,
        raw_ob: metar.raw_ob,
        name: metar.name,
        xwind: min_crosswind,
        hwind: best_headwind,
        density_altitude,
        best_runway,
        field_elevation: extracted_airport_data.field_elevation,
        diagram_link: extracted_airport_data.airport_diagram_link,
    };

    // Return the successful response
    Ok((StatusCode::OK, Json(metar_data_returned)))
}

async fn fetch_metar_data(code: &str) -> Option<MetarDataRaw> {
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
            let mut data: Vec<MetarDataRaw> = from_str(&body).ok()?;
            data.pop()
        })
}

async fn fetch_html(url: String) -> Option<String> {
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


// Import necessary crates
use chrono::{Datelike, NaiveDate, TimeZone, Utc, Local};
use scraper::{ElementRef, Html, Selector};

// Function to calculate crosswind component
fn calculate_crosswind(wind_dir: f64, wind_speed: f64, runway_heading: f64) -> f64 {
    let mut angle = (wind_dir - runway_heading).abs();
    if angle > 180.0 {
        angle = 360.0 - angle;
    }
    let crosswind = angle.to_radians().sin() * wind_speed;
    (crosswind * 10.0).round() / 10.0 // Round to one decimal place
}

// Function to calculate headwind component
fn calculate_headwind(wind_dir: f64, wind_speed: f64, runway_heading: f64) -> f64 {
    let mut angle = (wind_dir - runway_heading).abs();
    if angle > 180.0 {
        angle = 360.0 - angle;
    }
    let headwind = angle.to_radians().cos() * wind_speed;
    (headwind * 10.0).round() / 10.0 // Round to one decimal place
}

// Function to calculate pressure altitude
fn calculate_pressure_altitude(altimeter_setting: f64, field_elevation: f64) -> f64 {
    let standard_pressure = 1013.25;
    field_elevation + (standard_pressure - altimeter_setting) * 30.0
}

// Function to calculate density altitude
fn calculate_density_altitude(temp_c: f64, field_elevation: f64, altimeter_setting: f64) -> f64 {
    let pressure_altitude = calculate_pressure_altitude(altimeter_setting, field_elevation);
    let isa_temp = 15.0 - (0.00198 * field_elevation);
    let density_altitude = pressure_altitude + (120.0 * (temp_c - isa_temp));
    density_altitude.round() // Round to nearest whole number
}

// Function to convert hPa to inHg
fn hpa_to_inhg(hpa: f64) -> f64 {
    let conversion_factor = 0.029529983071445;
    let result = hpa * conversion_factor;
    (result * 100.0).round() / 100.0 // Round to 2 decimal places
}


// Function to convert Zulu time to local readable time
fn zulu_to_local_readable_time(zulu: &str) -> String {
    if zulu.len() < 6 {
        return "Invalid Zulu time format".to_string();
    }

    let day: u32 = zulu[0..2].parse().unwrap_or(1);
    let hour: u32 = zulu[2..4].parse().unwrap_or(0);
    let minute: u32 = zulu[4..6].parse().unwrap_or(0);

    let now = Utc::now();
    let year = now.year();
    let month = now.month();

    if let Some(naive_date) = NaiveDate::from_ymd_opt(year, month, day) {
        if let Some(naive_datetime) = naive_date.and_hms_opt(hour, minute, 0) {
            let utc_datetime = Utc.from_utc_datetime(&naive_datetime);
            let local_datetime = utc_datetime.with_timezone(&Local);
            local_datetime.format("%-I:%M %p").to_string() // Format as 'hour:minute AM/PM'
        } else {
            "Invalid time".to_string()
        }
    } else {
        "Invalid date".to_string()
    }
}


#[derive(Debug)]
struct Runway {
    number: String,           // e.g., "16R"
    heading_magnetic: f64,    // e.g., 166.0
}
#[derive(Debug)]
struct AirportInfo {
    field_elevation: f64,
    runways: Vec<Runway>,
    airport_diagram_link: Option<String>,
}


fn extract_airport_info(html_content: &str) -> Option<AirportInfo> {
    let document = Html::parse_document(html_content);

    // Check the <title> element to determine if the airport exists
    let title_selector = Selector::parse("title").ok()?;
    let title = document
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default()
        .trim()
        .to_string();

    if title == "AirNav: Airport Information" {
        // This indicates that the airport does not exist
        return None;
    }

    // Initialize variables
    let mut field_elevation = None;
    let mut runways = Vec::new();
    let mut airport_diagram_link = None;

    // Extract field elevation
    if let Some(location_h3) = document.select(&Selector::parse("h3").ok()?)
        .find(|h3| h3.text().collect::<Vec<_>>().join("").trim() == "Location")
    {
        if let Some(table_element) = location_h3.next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|e| e.value().name() == "table")
        {
            if let Some(elevation_tr) = table_element.select(&Selector::parse("tr").ok()?)
                .find(|tr| tr.text().collect::<Vec<_>>().join("").contains("Elevation:"))
            {
                let tds: Vec<_> = elevation_tr.select(&Selector::parse("td").ok()?).collect();
                if tds.len() >= 2 {
                    let elevation_str = tds[1].text().collect::<Vec<_>>().join("").trim().to_string();
                    if let Some(feet_part) = elevation_str.split("ft.").next() {
                        let feet_str = feet_part.trim();
                        if let Ok(feet_value) = feet_str.parse::<f64>() {
                            field_elevation = Some(feet_value);
                        } else if let Some((feet_value_str, _)) = feet_str.split_once(' ') {
                            if let Ok(feet_value) = feet_value_str.parse::<f64>() {
                                field_elevation = Some(feet_value);
                            }
                        }
                    }
                }
            }
        }
    }

    // Extract runway information
    if let Some(runway_info_h3) = document.select(&Selector::parse("h3").ok()?)
        .find(|h3| h3.text().collect::<Vec<_>>().join("").trim() == "Runway Information")
    {
        let mut sibling = runway_info_h3.next_sibling();
        while let Some(node) = sibling {
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if tag_name == "h3" {
                    break; // End of runway section
                } else if tag_name == "h4" {
                    let runway_header = element.text().collect::<Vec<_>>().join("").trim().to_string();
                    if runway_header.starts_with("Runway ") {
                        let runway_numbers = runway_header
                            .strip_prefix("Runway ")
                            .unwrap_or(&runway_header);

                        // Initialize headings to default values
                        let mut heading_magnetic = None;

                        // Now, extract the data within the table
                        if let Some(table_element) = element.next_siblings()
                            .filter_map(ElementRef::wrap)
                            .find(|e| e.value().name() == "table")
                        {
                            let tr_selector = Selector::parse("tr").ok()?;
                            for tr in table_element.select(&tr_selector) {
                                let tds: Vec<_> = tr.select(&Selector::parse("td").ok()?).collect();
                                if tds.len() >= 4 {
                                    let label = tds[0].text().collect::<Vec<_>>().join("").trim().to_string();
                                    if label.contains("Runway heading:") {
                                        // Extract headings for both runways
                                        let heading1 = tds[1].text().collect::<Vec<_>>().join("").trim().to_string();
                                        let heading2 = tds[3].text().collect::<Vec<_>>().join("").trim().to_string();

                                        // Parse magnetic headings
                                        if let Some(heading) = heading1.split("magnetic").next() {
                                            heading_magnetic = heading.trim().parse::<f64>().ok();
                                        }

                                        // Create Runway structs
                                        let runway_numbers: Vec<&str> = runway_numbers.split('/').collect();
                                        if runway_numbers.len() >= 2 {
                                            if let Some(heading) = heading_magnetic {
                                                runways.push(Runway {
                                                    number: runway_numbers[0].trim().to_string(),
                                                    heading_magnetic: heading,
                                                });
                                            }
                                            if let Some(heading) = heading2.split("magnetic").next()
                                                .and_then(|h| h.trim().parse::<f64>().ok())
                                            {
                                                runways.push(Runway {
                                                    number: runway_numbers[1].trim().to_string(),
                                                    heading_magnetic: heading,
                                                });
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            sibling = node.next_sibling();
        }
    }

    // Extract airport diagram link
    if let Some(font_element) = document.select(&Selector::parse("font").ok()?)
        .find(|font| font.text().collect::<Vec<_>>().join("").trim().contains("of official airport diagram from the FAA"))
    {
        let a_selector = Selector::parse("a").ok()?;
        if let Some(a_element) = font_element.select(&a_selector)
            .chain(
                font_element.parent().and_then(ElementRef::wrap)
                    .into_iter()
                    .flat_map(|e| e.select(&a_selector))
            )
            .chain(
                font_element.prev_siblings()
                    .filter_map(ElementRef::wrap)
                    .flat_map(|e| e.select(&a_selector))
            )
            .find(|a| a.value().attr("href")
                .map_or(false, |href| href.to_lowercase().ends_with(".pdf")))
        {
            airport_diagram_link = a_element.value().attr("href").map(|s| s.to_string());
        }
    }

    // Return the extracted airport information
    Some(AirportInfo {
        field_elevation: field_elevation.unwrap_or(0.0),
        runways,
        airport_diagram_link,
    })
}