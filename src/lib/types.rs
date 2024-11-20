use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetarDataRaw {
    metar_id: i64,
    icao_id: String,
    receipt_time: String,
    temp: f64,
    dewp: f64,
    wdir: String,
    wspd: i64,
    altim: f64,
    slp: f64,
    raw_ob: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct MetarDataReturned {
    metar_id: i64,
    icao_id: String,
    receipt_time: String,
    temp: f64,
    dewp: f64,
    wdir: String,
    wspd: i64,
    altimeter: i32,
    slp: f64,
    raw_ob: String,
    name: String,
}