use crate::config::DIST_UTILITY;
use serde::Deserialize;
use std::fs;

pub fn parse_map_json_data() {
    println!("{DIST_UTILITY}");
}

pub fn get_dist_utility() {
    println!("parsing distance utility json file");

    let contents = fs::read_to_string(DIST_UTILITY).expect("no such file exists");

    // let json = serde_json::from_str(&contents).unwrap();
    // println!("{}", json);
}

#[derive(Deserialize)]
struct DistUtility {}
