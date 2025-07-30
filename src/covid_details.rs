use rust_decimal::prelude::*;
use reqwest;
use serde::Deserialize;
use std::error::Error;

// Define a struct to match the structure of your JSON response
// We only need the fields we're interested in.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // Maps "camelCase" in JSON to "snake_case" in Rust
pub struct CovidData {
    pub(crate) cases: u64,
    pub(crate) deaths: u64,
    pub(crate) today_cases: u64,
    pub(crate) today_deaths: u64,
    pub(crate) today_recovered: u64,
}

pub fn get_covid_details() -> Result<CovidData,  Box<dyn std::error::Error>> {

    let api_url = "https://disease.sh/v3/covid-19/all";

    // Make the blocking HTTP GET request and parse the JSON response
    let json_data: CovidData = reqwest::blocking::get(api_url)?.json()?; // Use blocking::get and remove .await

    Ok(json_data)
}
