use std::f64::consts::PI;
use itertools::Itertools;
use axum::http::StatusCode;
use crate::types::{AirportInfo, Coordinates, MagneticVariation, MetarDataRaw, Runway, RunwayDataReturned, Summary, WindComponents, WindDirection, AMR, DMS};
use scraper::{ElementRef, Html, Selector};
use crate::errors::AppErrorExternal;
use crate::types;

pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}
fn signed_angle_difference(angle1: f64, angle2: f64) -> f64 {
    let angle1 = normalize_angle(angle1);
    let angle2 = normalize_angle(angle2);
    let mut diff = angle1 - angle2;

    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }

    diff
}
pub fn calculate_wind_components(wind_speed: f64, wind_direction: i32, runway_heading: i32) -> (f64, f64) {
    let angle_diff = signed_angle_difference(wind_direction as f64, runway_heading as f64);
    let angle_radians = angle_diff * PI / 180.0;

    let crosswind = ((wind_speed * angle_radians.sin()) * 100f64).round() / 100f64;
    let headwind = ((wind_speed * angle_radians.cos()) * 100f64).round() / 100f64;
    

    (crosswind, headwind)
}

pub fn calculate_pressure_altitude(altimeter_setting: f64, field_elevation: f64) -> f64 {
    let standard_pressure = 1013.25; // in hPa
    let pressure_altitude = field_elevation + (standard_pressure - altimeter_setting) * 30.0;

    // Round to two decimal places
    (pressure_altitude * 100.0).round() / 100.0
}


pub fn calculate_density_altitude(temp_c: f64, field_elevation: f64, altimeter_setting: f64) -> f64 {
    let pressure_altitude = calculate_pressure_altitude(altimeter_setting, field_elevation);
    let isa_temp = 15.0 - (0.00198 * field_elevation);
    let density_altitude = pressure_altitude + (120.0 * (temp_c - isa_temp));
    density_altitude.round() // Round to nearest whole number
}

// Function to convert hPa to inHg
pub fn hpa_to_inhg(hpa: f64) -> f64 {
    let conversion_factor = 0.029529983071445;
    let result = hpa * conversion_factor;
    (result * 100.0).round() / 100.0 // Round to 2 decimal places
}


// Function to convert Zulu time to local readable time
/*
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

 */
/*
pub fn build_runway_list(runways: Vec<Runway>, metar_data_raw: &MetarDataRaw) -> RunwayDataReturned {
    
    let wind_dir = metar_data_raw.wdir;
    
    for runway in runways {
        
    }
    
}

 */

/*
pub fn compute_wind_components(heading_magnetic: f64, wind_speed: i32, wind_direction: i32) -> WindComponents {
    
    /*
    let best_runway_info = match metar.wdir {
        WindDirection::Degree(v) => {
            let best_runway_info = extracted_airport_data.runways.iter()
                .map(|runway| {
                    let (crosswind, headwind) = calculate_wind_components(metar.wspd as f64, v, runway.heading_magnetic as i32);
                    let (gust_crosswind, gust_headwind) = match metar.wgst {
                        None => (None, None),
                        Some(gust_speed) => {
                            let (gust_crosswind, gust_headwind) = calculate_wind_components(gust_speed as f64, v, runway.heading_magnetic as i32);
                            (Some(gust_crosswind), Some(gust_headwind))
                        }
                    };

                    (runway, headwind, crosswind, runway.heading_magnetic, gust_crosswind, gust_headwind)
                })
                .max_by(|a, b| {
                    // First, compare headwinds
                    match a.1.partial_cmp(&b.1) {
                        Some(Ordering::Greater) => Ordering::Greater,
                        Some(Ordering::Less) => Ordering::Less,
                        Some(Ordering::Equal) => {
                            // If headwinds are equal, compare runway lengths
                            let a_length = a.0.length_ft;
                            let b_length = b.0.length_ft;
                            a_length.cmp(&b_length)
                        },
                        None => Ordering::Equal,
                    }
                })
                .ok_or((StatusCode::NOT_FOUND, "No runways available"))?;

            best_runway_info
        }
        WindDirection::Variable(_) => (&types::Runway { number: "Indeterminate".to_string(), heading_magnetic: 0f64, length_ft: 0i32 }, 0f64, 0f64, 0f64, None, None)
    };

    let (best_runway, best_headwind, corresponding_crosswind,best_runway_heading_mag, gust_crosswind, gust_headwind ) = best_runway_info;
    
     */

}

 */

pub fn extract_observation_time(metar: &str) -> String {
    // Split the METAR string into whitespace-separated tokens
    let tokens: Vec<&str> = metar.split_whitespace().collect();

    // The observation time is typically the second token
    if tokens.len() < 2 {
        return String::new();
    }

    let obs_time = tokens[1];

    // Check if the observation time matches the pattern: 6 digits followed by 'Z'
    if obs_time.len() == 7
        && obs_time[..6].chars().all(|c| c.is_digit(10))
        && obs_time.ends_with('Z')
    {
        obs_time.to_string()
    } else {
        String::new()
    }
}


pub fn extract_airport_info(html_content: &str) -> Option<AirportInfo> {
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
                        let runway_numbers_str = runway_header.strip_prefix("Runway ").unwrap_or(&runway_header);
                        let runway_numbers: Vec<&str> = runway_numbers_str.split('/').collect();

                        // Ensure we have two runway numbers
                        if runway_numbers.len() != 2 {
                            continue; // Skip if not a pair of runways
                        }

                        let runway_number_1 = runway_numbers[0].trim().to_string();
                        let runway_number_2 = runway_numbers[1].trim().to_string();

                        // Initialize variables for headings and length
                        let mut heading_magnetic_1 = None;
                        let mut heading_magnetic_2 = None;
                        let mut length_ft = None;

                        // Now, extract the data within the table
                        if let Some(table_element) = element.next_siblings()
                            .filter_map(ElementRef::wrap)
                            .find(|e| e.value().name() == "table")
                        {
                            let tr_selector = Selector::parse("tr").ok()?;
                            for tr in table_element.select(&tr_selector) {
                                let tds: Vec<_> = tr.select(&Selector::parse("td").ok()?).collect();
                                if tds.is_empty() {
                                    continue;
                                }

                                let label = tds[0].text().collect::<Vec<_>>().join("").trim().to_string();

                                if label.contains("Runway heading:") {
                                    // Expecting tds[1] and tds[3] for each runway direction
                                    if tds.len() >= 2 {
                                        // Runway 1 heading
                                        let heading_str = tds[1].text().collect::<Vec<_>>().join("").trim().to_string();
                                        // Extract the magnetic heading
                                        if let Some(mag_part) = heading_str.split("magnetic").next() {
                                            if let Ok(heading) = mag_part.trim().parse::<f64>() {
                                                heading_magnetic_1 = Some(heading);
                                            }
                                        }
                                    }
                                    if tds.len() >= 4 {
                                        // Runway 2 heading
                                        let heading_str = tds[3].text().collect::<Vec<_>>().join("").trim().to_string();
                                        if let Some(mag_part) = heading_str.split("magnetic").next() {
                                            if let Ok(heading) = mag_part.trim().parse::<f64>() {
                                                heading_magnetic_2 = Some(heading);
                                            }
                                        }
                                    }
                                } else if label.contains("Dimensions:") {
                                    // Dimensions are the same for both runways
                                    if tds.len() >= 2 {
                                        let dimensions_str = tds[1].text().collect::<Vec<_>>().join("").trim().to_string();
                                        if let Some(length_part) = dimensions_str.split("x").next() {
                                            let length_str = length_part.trim();
                                            if let Ok(length) = length_str.parse::<i32>() {
                                                length_ft = Some(length);
                                            }
                                        }
                                    }
                                }
                                // You can handle other labels here if needed
                            }
                        }

                        // Now, create Runway structs if headings and length are available
                        if let (Some(heading1), Some(heading2), Some(length)) = (heading_magnetic_1, heading_magnetic_2, length_ft) {
                            runways.push(Runway {
                                number: runway_number_1,
                                heading_magnetic: heading1,
                                length_ft: length,
                            });
                            runways.push(Runway {
                                number: runway_number_2,
                                heading_magnetic: heading2,
                                length_ft: length,
                            });
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

pub fn extract_airport_html(id: &str, html: &str) -> Result<AMR, AppErrorExternal> {

    let mut data = AMR {
        identifier: id.to_uppercase(),
        units: "FAA".to_string(),
        summary: Summary {
            coordinates: Coordinates {
                latitude: DMS {
                    degrees: 0,
                    minutes: 0,
                    seconds: 0.0,
                    direction: "N".to_string(),
                },
                longitude: DMS {
                    degrees: 0,
                    minutes: 0,
                    seconds: 0.0,
                    direction: "W".to_string(),
                },
            },
            elevation: 0.0,
            magnetic_variation: MagneticVariation {
                value: 0,
                direction: "N".to_string(),
                year: 0,
            },
            nearby_city: "".to_string(),
            artcc: "".to_string(),
            sectional_chart: "".to_string(),
            tz: "".to_string(),
        },
    };
    
    let document = Html::parse_document(html);
    
    let summary_table = document
        .select(&Selector::parse("#summary table.table.table-condensed.table-borderless")?)
        .next()
        .ok_or(AppErrorExternal::Custom("Could not find the summary table".to_string()))?;

    let row_selector = Selector::parse("tr")?;
    let cell_selector = Selector::parse("td")?;

    let mut lat_lon_raw = String::new();
    let mut magvariation_raw = String::new();
    
    for row in summary_table.select(&row_selector) {
        // The first <td> is our label, the second <td> is the value
        let mut cells = row.select(&cell_selector);
        let label = cells
            .next()
            .map(|td| td.text().collect::<String>().trim().to_lowercase());
        let value = cells
            .next()
            .map(|td| td.text().collect::<String>().replace('\n', " ").trim().to_string());

        //dbg!(&label, &value);

        if let (Some(label), Some(value)) = (label, value) {
            match label.as_str() {
                "latitude/longitude" => lat_lon_raw = value,
                "elevation" => data.summary.elevation = value.chars().filter(|c| c.is_numeric() || *c == '.').collect::<String>().parse()?,
                "variation" => magvariation_raw = value,
                "from city" => data.summary.nearby_city = value.split_whitespace().intersperse(" ").collect::<String>(),
                "artcc" => data.summary.artcc = value,
                "section chart" => data.summary.sectional_chart = value,
                "time zone" => data.summary.tz = value,
                _ => {}
            }
        }
    }

    { // parse the lag long
        let parts: Vec<&str> = lat_lon_raw.split(" / ").collect();

        let coordinates = Coordinates {
            latitude: parse_dms(parts[0])?,
            longitude: parse_dms(parts[1])?,
        };
        
        data.summary.coordinates = coordinates;
    }

    { // parse mag variation
        let parts: Vec<&str> = magvariation_raw.split_whitespace().collect();

        data.summary.magnetic_variation = MagneticVariation {
            value: parts[0].parse().expect("Invalid value"),
            direction: parts[1].to_string(),
            year: parts[2].parse().expect("Invalid year"),
        }
    }


        Ok(data)
}

fn parse_dms(dms_str: &str) -> Result<DMS, AppErrorExternal> {
    let parts: Vec<&str> = dms_str.trim().split_whitespace().collect();
    let coords: Vec<&str> = parts[0].split('-').collect();

    Ok(DMS {
        degrees: coords[0].parse()?,
        minutes: coords[1].parse()?,
        seconds: coords[2].parse()?,
        direction: parts[1].to_string(),
    })
}

fn parse_magnetic_variation(input: &str) -> Result<MagneticVariation, AppErrorExternal> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    Ok(MagneticVariation {
        value: parts[0].parse()?,
        direction: parts[1].to_string(),
        year: parts[2].parse()?,
    })
}