use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use serde::{Serialize, Deserialize};
use serde_json::Result;

const ACTIVE_PROFILE: &str = "ACTIVE_PROFILE";
const DIRECTORY: &str = "resources/config/";


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    engine: Engine
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    starting_cash:  f64,
    symbols:        Vec<String>,
    timeframes:     Vec<String>,
    exchanges:      Vec<String>
}

pub fn load_config() -> Config {
    // Todo: Add validation function for the passed config

    // Extract ACTIVE_PROFILE environment variable
    let active_profile = match env::var(ACTIVE_PROFILE) {
        Ok(value) => value,
        Err(_error) => String::from("default")
    };

    // Open config file & use to create a buffered reader
    let file_path = format!("{}{}{}", DIRECTORY, active_profile, ".json");
    let mut reader = match File::open(file_path) {
        // Ok(file) => file,
        Ok(config_file) => BufReader::new(config_file),
        Err(error) => panic!("failed to open file: {:?}", error)
    };

    // Read in config file contents to String
    let mut config_contents = String::new();
    match reader.read_to_string(&mut config_contents) {
        Ok(bytes_read) => bytes_read,
        Err(err) => panic!("failed to read config file contents to String: {}", err)
    };

    // Map config json String to Config struct & return Result<Config>
    let config: Config = match serde_json::from_str(&*config_contents) {
        Ok(config) => config,
        Err(err) => panic!("failed to init environment config: {}", err)
    };

    return config;
}
