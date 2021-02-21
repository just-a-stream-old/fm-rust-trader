use crate::model::{SymbolData, Bar, MarketEvent, EVENT_TYPE_MARKET};
use crate::config::{Trader};

const DATA_DIRECTORY: &str = "resources/data/";

pub struct HistoricDataHandler {
    event_q:                Vec<MarketEvent>,
    symbol:                 String,
    all_symbol_data:        SymbolData,
    current_symbol_data:    SymbolData,
    latest_bar_index:       usize
}

impl HistoricDataHandler {
    pub fn new(cfg: &Trader, event_q: Vec<MarketEvent>) -> HistoricDataHandler {
        // Todo: log debug file path
        // Todo: Improve error handling, pass back to main with added info like Go? errors.wrap()
        let all_symbol_data = match load_csv_symbol_data(cfg) {
            Ok(data) => data,
            Err(err) => panic!("{}", err)
        };

        HistoricDataHandler{
            event_q,
            symbol: cfg.symbol.clone(),
            all_symbol_data,
            current_symbol_data: SymbolData::default(),
            latest_bar_index: 0, // Todo: Fix implications of this, but it's the right thing to do
        }
    }

    pub fn should_continue(&self) -> bool {
        // Return true if the next latest_bar_index has Some data instead of None
        self.all_symbol_data.timestamps
            .get(self.latest_bar_index + 1)
            .is_some()
    }

    pub fn update_data(&mut self) {
        // Increment latest_bar_index
        self.latest_bar_index += 1;

        // Add latest bar to current_symbol_data (index checked in should_continue so is safe)
        let latest_bar = Bar{
            timestamp: self.all_symbol_data.timestamps[self.latest_bar_index],
            open: self.all_symbol_data.opens[self.latest_bar_index],
            high: self.all_symbol_data.highs[self.latest_bar_index],
            low: self.all_symbol_data.lows[self.latest_bar_index],
            close: self.all_symbol_data.closes[self.latest_bar_index],
            volume: self.all_symbol_data.volumes[self.latest_bar_index],
        };
        self.current_symbol_data.add_bar(&latest_bar);

        // Add MarketEvent to the queue
        self.event_q.push(MarketEvent{
            event_type: EVENT_TYPE_MARKET,
            trace_id: String::from("add uuid here"),
            timestamp: latest_bar.timestamp,
            symbol: self.symbol.clone(),
            close: latest_bar.close,
        });
    }

    pub fn get_latest_data(&self) -> (&SymbolData, usize) {
        (&self.current_symbol_data, self.latest_bar_index)
    }
}

// Todo: Move func to lib?
fn load_csv_symbol_data(cfg: &Trader) -> Result<SymbolData, csv::Error> {
    // Read the file
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(build_csv_file_path(cfg))?;

    // Iterate through the lines & add the deserialised Bar struct to symbol_data
    let mut symbol_data = SymbolData::default();

    for result_bar in reader.deserialize() {
        symbol_data.add_bar(&result_bar?);
    }

    return Ok(symbol_data)
}

// Todo: Move func to lib?
fn build_csv_file_path(cfg: &Trader) -> String {
    format!("{}{}_{}.csv", DATA_DIRECTORY, cfg.symbol, cfg.timeframe)
}