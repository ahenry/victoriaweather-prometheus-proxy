extern crate metrics;
extern crate hyper;
extern crate xml;

use hyper::client::Client;
use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}


fn main() {
    let client = Client::new();

    let res = client.get("http://www.victoriaweather.ca/stations/Lighthouse/current.xml")
        .send()
        .unwrap();
    if res.status == hyper::Ok {
        println!("{:?}", res);
        let parser = EventReader::new(res);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{}+{}", indent(depth), name);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{}-{}", indent(depth), name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}
