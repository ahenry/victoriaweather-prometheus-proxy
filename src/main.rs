#![feature(proc_macro)]

extern crate clap;
extern crate hyper;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml;

use clap::{Arg, App};
use hyper::client::{Client, IntoUrl};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::mime::Mime;
use std::io::Read;

use prometheus::{Gauge, Encoder, TextEncoder};

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
struct current_observation {
    station_long_name: String,
    station_name: String,
    station_id: String,

    observation_time: String, // TODO needs to be a date type
    timezone: String,

    temperature: f64,
    temperature_low: f64,
    temperature_high: f64,
    temperature_units: String,

    humidity: f64,
    humidity_units: String,

    insolation: f64,
    insolation_units: String,
    insolation_predicted: f64,
    insolation_predicted_units: String,
}

#[allow(dead_code)]
fn get_current_conditions<U: IntoUrl>(url: U) -> Option<current_observation> {
    let mut res = Client::new().get(url).send().unwrap();

    if res.status == hyper::Ok {
        if let Some(&ContentLength(length)) = res.headers.get() {
            let mut content = String::with_capacity(length as usize);
            let _ = res.read_to_string(&mut content);
            let readings: current_observation = serde_xml::from_str(&content).unwrap();
            return Some(readings);
        }
    }

    None
}

fn is_int(v: String) -> Result<(), String> {
    if v.parse::<u16>().is_ok() {
        return Ok(());
    }

    Err(String::from("The value needs to be a positive integer less than 65535"))
}

fn main() {
    let matches = App::new("Victoria Weather Prometheus Exporter")
        .version("0.1")
        .author("Austin Henry <ahenry@twocanoe.ca>")
        .about("Does what you'd expect from the name")
        .arg(Arg::with_name("port")
            .short("p")
            .long("listen_port")
            .help("Which port to listen on")
            .takes_value(true)
            .default_value("9189")
            .validator(is_int))
        .arg(Arg::with_name("location")
            .short("l")
            .long("location")
            .help("The short name of the victoria weather station to use")
            .takes_value(true)
            .required(true))
        .get_matches();

    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let location = matches.value_of("location").unwrap();
    let url = format!("http://www.victoriaweather.ca/stations/{}/current.xml", location);
    /*
    let readings = current_observation {
        station_long_name: "Lighthouse Christian Academy".into(),
        station_name: "Lighthouse".into(),
        station_id: "169".into(),
        observation_time: "2016/11/02, 15:21".into(),
        timezone: "Pacific".into(),
        temperature: 11.8,
        temperature_low: 9.3,
        temperature_high: 11.8,
        temperature_units: "C".into(),
        humidity: 97.0,
        humidity_units: "%".into(),
        insolation: 100.0,
        insolation_units: "W/m2".into(),
        insolation_predicted: 290.0,
        insolation_predicted_units: "W/m2".into(),
    };
    */

    let temperature: Gauge =
        register_gauge!("thermostat_temperature",
                        "The temperature in degrees C at the location",
                        labels!{"location" => location,})
            .unwrap();

    let humidity: Gauge =
        register_gauge!("thermostat_humidity",
                        "The humidity in % at the location",
                        labels!{"location" => location,})
            .unwrap();

    let insolation: Gauge =
        register_gauge!("thermostat_insolation",
                        "The insolation in degrees W/m2 at the location",
                        labels!{"location" => location,})
            .unwrap();

    let encoder = TextEncoder::new();
    println!("listening addr 127.0.0.1:{}", port);
    Server::http(("127.0.0.1", port))
        .unwrap()
        .handle(move |_: Request, mut res: Response| {
            if let Some(readings) = get_current_conditions(&url) {
                temperature.set(readings.temperature);
                humidity.set(readings.humidity);
                insolation.set(readings.insolation);

                let metric_familys = prometheus::gather();
                let mut buffer = vec![];
                encoder.encode(&metric_familys, &mut buffer).unwrap();
                res.headers_mut()
                    .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
                res.send(&buffer).unwrap();
            } else {
                *res.status_mut() = StatusCode::BadGateway;
            }

        })
        .unwrap();
}
