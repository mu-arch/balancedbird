use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub enum WindDirection {
    Degree(i64),
    Variable(String),
}

#[derive(Debug, Deserialize)]
pub enum WindSpeed {
    Degree(i64),
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetarDataReturned {
    pub(crate) metar_id: i64,
    pub(crate) temp: f64,
    pub(crate) dewp: f64,
    pub(crate) wdir: i64,
    pub(crate) wspd: i32,
    pub(crate) xwind: f64,
    pub(crate) hwind: f64,
    pub(crate) pressure_altitude: f64,
    pub(crate) density_altitude: f64,
    pub(crate) altimeter: f64,
    pub(crate) raw_ob: String,
    pub(crate) name: String,
    pub(crate) best_runway: String,
    pub(crate) field_elevation: f64,
    pub(crate) diagram_link: Option<String>,
    pub(crate) runway_length: i32,
}