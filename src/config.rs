use serde::{Deserialize, Serialize};

use crate::Locations;

#[derive(Serialize, Deserialize)]
pub struct Config {
    title: String,
}

impl Config {
    pub fn new(locations: Locations) -> Self {
        if !locations.config.exists() {
            panic!(
                "Config file does not exist, tried: {}",
                locations.config.display()
            );
        }

        println!(
            "{}",
            std::fs::read_to_string(locations.config.clone()).unwrap()
        );
        serde_json::from_str(std::fs::read_to_string(locations.config).unwrap().as_str()).unwrap()
    }
}
