//!
//! ```
//! asq [-v] IP-ADDRESS
//! ```
//!
//! Command line command to tell AS of an IP address.
//! AS, Autonomous System is the top level of the IP
//! routing in the Internet.
//! AS usually means the international operator of the IP address.
//!
//! # Example
//! (at command shell)
//! ```bash
//! $ host -4 -t A ibm.com
//! ibm.com has address 129.42.38.10
//! $ asq 129.42.38.10
//! IBM-EI
//! $ asq -v 129.42.38.10
//! IBM-EI - IBM - Events Infrastructure - US
//! IBMCCH-RTP - IBM - US
//! ISSC-AS - IBM Corporation - US
//! $ host -4 -t A www.ibm.com
//! ...
//! ... has address 59.151.164.181
//! $ asq 59.151.164.181
//! AKAMAI-AS
//! $ asq -v 59.151.164.181
//! AKAMAI-AS - Akamai Technologies, Inc. - US
//! AKAMAI-TYO-AP - Akamai Technologies Tokyo ASN - SG
//!```
//!
//! # Installation
//!
//! ```cargo install --path .```
//!
//! installs the asq command to ~/.cargo/bin
//!

#[macro_use] extern crate clap;
extern crate reqwest;
extern crate exitcode;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};
use serde::Deserialize;

fn main() {

    let matches = App::new("asq")
        .version("0.0.1")
        .author(crate_authors!())
        .about("Query AS of an IP address")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Show more information"),
        )
        .arg(
            Arg::with_name("ip_address")
                .index(1)
                .required(true)
                .help("IP address whose AS we want to know"),
        )
        .get_matches();

    let verbose: bool = matches.is_present("verbose");
    // value_of("ip_address") can not fail because 'required'
    let ip_address = matches.value_of("ip_address").unwrap().to_string();

    let response = get_as(ip_address);
    let as_response = match response {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::UNAVAILABLE);
        },
        Ok(v) => v,
    };
    if as_response.status != "ok" {
        eprintln!("Error: Server responded {}", as_response.status);
        std::process::exit(exitcode::UNAVAILABLE);
    };
    if !verbose {
        println!("{}", as_response.data.prefixes[0].asn.name);
    }
    else {
        let prefixes = as_response.data.prefixes;
        for prefix in prefixes {
            println!("{} - {} - {}", prefix.asn.name, prefix.asn.description, prefix.asn.country_code)
        }
    }
    std::process::exit(exitcode::OK);

}

#[derive(Deserialize, Debug)]
struct AsResponse {
    status: String,
    status_message: String,
    data: AsData,
}
#[derive(Deserialize, Debug)]
struct AsData {
    prefixes: Vec<AsPrefix>,
}
#[derive(Deserialize, Debug)]
struct AsPrefix {
    prefix: String,
    ip: String,
    cidr: u32,
    asn: AsAsn,
    name: String,
    description: String,
    country_code: String,
}
#[derive(Deserialize, Debug)]
struct AsAsn {
    asn: u32,
    name: String,
    description: String,
    country_code: String,
}

/// Connects to bgpview.io to fill AsResponse struct
fn get_as(ip_address: String) -> Result<AsResponse, String> {
    let url = format!("https://api.bgpview.io/ip/{}", ip_address);
    let json_text = match GET(&url) {
        Ok(json_text) => json_text,
        Err(e) => return Err(format!("Could not connect to bgpview.io, {}", e)),
    };
    let json: serde_json::Result<AsResponse> = serde_json::from_str(&json_text);
    match json {
        Ok(as_response) => Ok(as_response),
        Err(e) => Err(format!("Could not parse JSON response, {}, {}", e, json_text)),
    }
}

/// Make get request to URL
// This is a separate function because I can not figure out
// better way to return error. Both question marks must
// return Reqwest::Result.
#[allow(non_snake_case)]
#[doc(hidden)]
fn GET(url: &str) -> reqwest::Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}
