#![feature(proc_macro)]

extern crate metrics;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml;

use hyper::client::{Client, IntoUrl};
use hyper::header::ContentLength;
use std::io::Read;

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
struct current_observation {
    station_long_name: String,
    station_name: String,
    station_id: String,

    observation_time: String, // TODO needs to be a date type
    timezone: String,

    temperature: f32,
    temperature_low: f32,
    temperature_high: f32,
    temperature_units: String,

    humidity: f32,
    humidity_units: String,

    insolation: f32,
    insolation_units: String,
    insolation_predicted: f32,
    insolation_predicted_units: String,
}

fn get_current_conditions<U: IntoUrl>(url: U) -> Option<current_observation> {
    let mut res = Client::new().get(url).send().unwrap();

    if res.status == hyper::Ok {
        if let Some(&ContentLength(length)) = res.headers.get() {
            let mut content = String::with_capacity(length as usize);
            let _ = res.read_to_string(&mut content);
            // println!("{}", content);
            let readings: current_observation = serde_xml::from_str(&content).unwrap();
            return Some(readings);
        }
    }

    None
}

fn main() {
    let url = "http://www.victoriaweather.ca/stations/Lighthouse/current.xml";
    println!("{:?}", get_current_conditions(url));
}

// This is the full set of fields
// #[allow(non_camel_case_types)]
// struct current_observation {
// credit: String,
// credit_url: String,
// description: String,
// disclaimer: String,
// station_long_name: String,
// station_name: String,
// station_id: String,
// latitude: f32,
// longitude: f32,
// elevation: f32,
// observation_time: String, // TODO needs to be a date type
// timezone: String,
//
// temperature: f32,
// temperature_low: f32,
// temperature_high: f32,
// temperature_units: String,
//
// humidity: f32,
// humidity_units: String,
//
// dewpoint: f32,
// dewpoint_units: String,
//
// wetbulb: f32,
// wetbulb_units: String,
// pressure: f32,
// pressure_units: String,
// pressure_trend: String,
// insolation: f32,
// insolation_units: String,
// uv_index: f32,
// uv_index_units: String,
// rain: f32,
// rain_units: String,
// rain_rate: f32,
// rain_rate_units: String,
// wind_speed: f32,
// wind_speed_direction: f32,
// wind_speed_heading: String,
// wind_speed_direction_units: String,
// wind_speed_max: f32,
// wind_speed_units: String,
// evapotranspiration: f32,
// evapotranspiration_units: String,
// insolation_predicted: f32,
// insolation_predicted_units: String,
// }
//
