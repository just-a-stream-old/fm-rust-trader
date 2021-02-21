use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use serde::{Serialize, Deserialize};

const ACTIVE_PROFILE: &str = "ACTIVE_PROFILE";
const CONFIG_DIRECTORY: &str = "resources/config/";


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    engine: Engine,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    starting_cash:  f64,
    symbols:        Vec<String>,
    timeframes:     Vec<String>,
    exchanges:      Vec<String>,
}

#[derive(Debug)]
pub struct Trader {
    // Todo: Add logger
    // ticker symbol this instance of Trader is using
    pub symbol:              String,
    // interval between bars this instance of Trader is using
    pub timeframe:           String,
    // name of the exchange this instance of Trader is using
    pub exchange:            String,
    // starting capital allocated to this instance of Trader
    pub starting_cash:       f64,
    // default value used by the SizeManager to determine the quantity of an order
    pub default_order_value: f64,
}

pub fn load_config() -> Config {
    // Todo:
    //  - Add validation function for the passed config
    //  - Do I return a Result<Config> or not? Handle errors or propagate with '?' ?
    //  - Extract methods where applicable

    // Extract ACTIVE_PROFILE environment variable
    let active_profile = match env::var(ACTIVE_PROFILE) {
        Ok(value) => value,
        Err(_error) => String::from("default")
    };

    // Open config file & use to create a buffered reader
    let file_path = format!("{}{}{}", CONFIG_DIRECTORY, active_profile, ".json");
    let mut reader = match File::open(file_path) {
        // Ok(file) => file,
        Ok(config_file) => BufReader::new(config_file),
        Err(error) => panic!("failed to open file: {}", error)
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
        Err(err) => panic!("failed to map config file contents to Config struct: {}", err)
    };

    return config;
}
