mod config;
mod model;
mod data;

use crate::config::*;
use crate::model::{Bar, SymbolData};

fn main() {
    // if let Err(err) = run() {}

    // Load environment configuration
    let config = load_config();

    let symbol_data = load_csv_symbol_data(
        String::from("resources/data/ETH-USD_1D.csv")
    ).unwrap();

    println!("{:?}", symbol_data);

}