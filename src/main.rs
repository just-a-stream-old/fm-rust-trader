mod config;
mod model;
mod data;

use crate::config::*;
use crate::data::HistoricDataHandler;

fn main() {
    // if let Err(err) = run() {}

    // Load environment configuration
    let config = load_config();

    // Create data handler
    // let trader_config = Trader{
    //     symbol: "ETH-USD".to_string(),
    //     timeframe: "1D".to_string(),
    //     exchange: "BINANCE".to_string(),
    //     starting_cash: 0.0,
    //     default_order_value: 0.0
    // };
    // let data_handler = HistoricDataHandler::new(&trader_config, vec![]);
    // let all_symbol_data = data_handler.all_symbol_data;
    // println!("{:#?}", all_symbol_data);

}