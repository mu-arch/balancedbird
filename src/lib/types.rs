use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] //By default, Serde expects enums to be internally tagged, meaning the JSON should specify which variant it is using. Because the JSON from aviationweather.gov doesn't provide this tagging we need to inform serde we can't do it with this line
pub enum WindDirection {
    Degree(i32),
    Variable(String),
}

#[derive(Debug, Deserialize)]
pub struct MetarDataRaw {
    pub metar_id: i64,
    pub temp: f64,
    pub dewp: f64,
    pub wdir: WindDirection,
    pub wspd: i32,
    pub altim: f64,
    #[serde(rename = "rawOb")]
    pub raw_ob: String,
    pub name: String,
    pub wgst: Option<i32>
}

struct Metar {
    wdir: WindDirection,
    wspd: f64, 
    gust: Option<f64>, 
    peak_wind_dir: Option<i32>,
    peak_wind_speed: Option<f64>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherDataReturned {
    pub(crate) metar_id: i64,
    pub(crate) temp: f64,
    pub(crate) dewp: f64,
    pub(crate) wdir: WindDirection,
    pub(crate) wspd: i32,
    pub wgst: Option<i32>,
    pub(crate) xwind: f64,
    pub(crate) hwind: f64,
    pub(crate) gxwind: Option<f64>,
    pub(crate) ghwind: Option<f64>,
    pub(crate) pressure_altitude: f64,
    pub(crate) density_altitude: f64,
    pub(crate) altimeter: f64,
    pub(crate) raw_ob: String,
    pub(crate) obs_time: String,
    pub(crate) server_time: String,
    pub(crate) name: String,
    pub(crate) best_runway: String,
    pub(crate) best_runway_heading_magnetic: f64,
    pub(crate) field_elevation: f64,
    pub(crate) diagram_link: Option<String>,
    pub(crate) runway_length: i32,
}

#[derive(Debug)]
pub struct Runway {
    pub(crate) number: String,           // e.g., "18L"
    pub(crate) heading_magnetic: f64,    // e.g., 177.0
    pub(crate) length_ft: i32,           // e.g., 7002
}
#[derive(Debug)]
pub struct AirportInfo {
    pub(crate) field_elevation: f64,
    pub(crate) runways: Vec<Runway>,
    pub(crate) airport_diagram_link: Option<String>,
}
