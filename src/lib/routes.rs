use axum::extract::Path;
use axum::http::{StatusCode};
use axum::Json;
use crate::{helper, types};
use crate::rx;
use chrono::{Datelike, NaiveDate, TimeZone, Utc, Local};
use serde_json::json;
use scraper::{Html, Selector};


use crate::types::{Coordinates, MagneticVariation, Summary, WeatherDataReturned, WindDirection, AMR, DMS};

// Handler function for the /weather/:code route
//pub async fn weather_handler(Path(code): Path<String>) -> Result<(StatusCode, Json<WeatherDataReturned>), (StatusCode, &'static str)> {

/*
pub async fn weather_handler(Path(code): Path<String>)
                             -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, &'static str)> {

    // Fetch METAR step
    //note on all these return types we need to use INTERNAL_SERVER_ERROR otherwise we get CORS errors
    let mut metar = rx::fetch_metar_data(&code).await
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch METAR data. That airport may not exist or isn't currently reporting."))?;

    
    
    // Fetch airport data from Airnav step
    let airnav_page_data = rx::fetch_html(format!("https://www.airnav.com/airport/{}", &code))
        .await
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Network error while downloading airport data"))?;

    // Extract airport information from the scraper
    let extracted_airport_data = helper::extract_airport_info(&airnav_page_data)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Airport not found"))?;

    // Perform air pressure calculations
    let pressure_altitude = helper::calculate_pressure_altitude(metar.altim, extracted_airport_data.field_elevation);
    let altimeter_inhg = helper::hpa_to_inhg(metar.altim);
    let density_altitude = helper::calculate_density_altitude(
        metar.temp,
        extracted_airport_data.field_elevation,
        metar.altim,
    );

    // Determine the best runway based on crosswind
    
    //let runways = build_runway_list(extracted_airport_data.runways, &metar);


    // Prepare the response data
    // if the wind dir was VRB ignore basically all the values in the response because they will be zero or wrong because they do not apply at all since we cannot compute anything when with is VRB
    
    /*
    let metar_data_returned = WeatherDataReturned {
        metar_id: metar.metar_id,
        temp: metar.temp,
        dew_point: metar.dewp,
        wind_direction: metar.wdir,
        wind_speed_base: metar.wspd,
        wind_gust: metar.wgst,
        altimeter: altimeter_inhg,
        raw_ob: metar.raw_ob.clone(),
        obs_time: extract_observation_time(&metar.raw_ob),
        server_time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string(),
        name: metar.name,
        xwind: corresponding_crosswind,
        hwind: best_headwind,
        gxwind: gust_crosswind,
        ghwind: gust_headwind,
        pressure_altitude,
        density_altitude,
        best_runway: best_runway.number.clone(),
        best_runway_heading_magnetic: best_runway_heading_mag,
        runway_length: best_runway.length_ft,
        field_elevation: extracted_airport_data.field_elevation,
        diagram_link: extracted_airport_data.airport_diagram_link,
    };
    
     */

    //Ok((StatusCode::OK, Json(metar_data_returned)))
    Ok((StatusCode::OK, Json(json!({}))))
}

 */

pub async fn amr_handler(Path(code): Path<String>)
                         -> Result<(StatusCode, Json<AMR>), (StatusCode, &'static str)> {

    let html = rx::fetch_html(format!("https://nfdc.faa.gov/nfdcApps/services/ajv5/airportDisplay.jsp?airportId={}", &code))
        .await
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Network error while downloading airport data"))?;

    let json = match helper::extract_airport_html(&code, &html) {
        Ok(json) => json,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error parsing airport data")),
    };
    
    Ok((StatusCode::OK, Json(json)))
}
