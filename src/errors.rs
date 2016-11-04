use hyper;
use serde_xml;
use std;

error_chain! {
    foreign_links {
        std::io::Error, IO;
        hyper::error::Error, Hyper;
        serde_xml::Error, Serde;
    }
}
