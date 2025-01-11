use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] //By default, Serde expects enums to be internally tagged, meaning the JSON should specify which variant it is using. Because the JSON from aviationweather.gov doesn't provide this tagging we need to inform serde we can't do it with this line
pub enum WindDirection {
    Degree(i32),
    Variable(String),
}


#[derive(Debug, Serialize)]
pub struct WeatherDataReturned {
    pub metar_id: i64,
    pub temp: f64,
    pub dew_point: f64,
    pub wind_direction: WindDirection,
    pub wind_speed: i32,//normal wind
    pub wind_gust: Option<i32>,//gust wind
    pub wind_peak: Option<i32>,//peak wind
    pub pressure_altitude: f64,
    pub density_altitude: f64,
    pub altimeter: f64,
    pub raw_observation: String,
    pub observation_time: String,
    pub name: String, // airport name ex Denton Muni, TX, US
    pub field_elevation: f64,
    pub diagram_link: Option<String>,
    
    pub balancedbird_server_time: String,
    pub balancedbird_tz: String,
    
    pub runways: Vec<RunwayDataReturned>
}

#[derive(Debug, Serialize)]
pub struct RunwayDataReturned {
    pub(crate) number: String,           // ex "18L"
    pub(crate) heading_magnetic: f64,    // ex 177.0
    pub(crate) length_ft: i32,           // ex 7002
    pub(crate) best_runway: bool,
    pub wind: RunwayWindComponents,
}

#[derive(Debug, Serialize)]
pub struct RunwayWindComponents {
    steady: WindComponents,
    gust: Option<WindComponents>,
    peak: Option<WindComponents>,
}

#[derive(Debug, Serialize)]
pub struct WindComponents {
    headwind: f64,
    crosswind: f64,
}





// internal types
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

// raw deserialize types

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
    pub wgst: Option<i32>,
    pub wpk: Option<i32>
}

struct Metar {
    wdir: WindDirection,
    wspd: f64,
    gust: Option<f64>,
    peak_wind_dir: Option<i32>,
    peak_wind_speed: Option<f64>,
}