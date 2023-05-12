use crate::config::DIST_UTILITY;
use anyhow::Result;

use std::{collections::HashMap, fs};

pub fn get_dist_utility() -> Result<HashMap<String, HashMap<String, f32>>> {
    println!("parsing distance utility json file");

    let contents = fs::read_to_string(DIST_UTILITY)?;

    let json: HashMap<String, HashMap<String, f32>> = serde_json::from_str(&contents)?;

    Ok(json)
}
