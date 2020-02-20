///
/// Admin command line tool for Crush servers
///
///

#[macro_use] extern crate clap;
extern crate reqwest;
extern crate exitcode;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};
use serde::Deserialize;

///
/// asq
/// Command line command to tell AS of and IP address
/// AS, Autonomous System is the top level of the IP routing in the Internet
/// AS usually means the international operator of the IP address
///
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

// connects to bgpview to fill AsResponse
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

// This is a separate function because I can not figure out
// better way to return error. Both question marks must
// return Reqwest::Result.
#[allow(non_snake_case)]
fn GET(url: &str) -> reqwest::Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}
