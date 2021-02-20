use crate::model::{SymbolData, Bar, MarketEvent, EVENT_TYPE_MARKET};

const DATA_DIRECTORY: &str = "resources/data/";

pub struct HistoricDataHandler {
    event_q:                Vec<MarketEvent>,
    symbol:                 String,
    all_symbol_data:        SymbolData,
    current_symbol_data:    SymbolData,
    latest_bar_index:       i64
}

impl HistoricDataHandler {
    fn should_continue(&self) -> bool {
        // Return true if the latest_bar_index has not reached the end of the data
        self.latest_bar_index < ((self.all_symbol_data.timestamps.len() as i64) - 1)
    }

    fn update_data(&mut self) {
        // Increment latest_bar_index
        self.latest_bar_index += 1;

        // Add latest bar to current_symbol_data
        let latest_bar = Bar{
            timestamp: self.all_symbol_data.timestamps[self.latest_bar_index],
            open: self.all_symbol_data.opens[self.latest_bar_index],
            high: self.all_symbol_data.highs[self.latest_bar_index],
            low: self.all_symbol_data.lows[self.latest_bar_index],
            close: self.all_symbol_data.closes[self.latest_bar_index],
            volume: self.all_symbol_data.volumes[self.latest_bar_index],
        };
        self.current_symbol_data.add_bar(latest_bar);

        // Add MarketEvent to the queue
        self.event_q.push(MarketEvent{
            event_type: EVENT_TYPE_MARKET,
            trace_id: String::from("add uuid here"),
            timestamp: latest_bar.timestamp,
            symbol: self.symbol,
            close: latest_bar.close,
        });
    }
}