use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub const EVENT_TYPE_MARKET: String = String::from("MARKET_EVENT");
const EVENT_TYPE_SIGNAL: String = String::from("SIGNAL_EVENT");
const EVENT_TYPE_ORDER: String = String::from("ORDER_EVENT");
const EVENT_TYPE_FILL: String = String::from("FILL_EVENT");

pub struct MarketEvent {
    pub event_type: String,
    pub trace_id: String,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub close: f64,
}

// Bar represents a symbol's market data state at a fixed interval of time
pub struct Bar {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

// SymbolData represents a symbol's struct of market data arrays (OHLCV) and associated indicators values
pub struct SymbolData {
    pub timestamps: Vec<DateTime<Utc>>,
    pub opens: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<f64>,
    pub indicators: HashMap<String, Vec<f64>>,
}

impl SymbolData {
    pub fn add_bar(&mut self, bar: Bar) -> &mut SymbolData {
        self.timestamps.push(bar.timestamp);
        self.opens.push(bar.open);
        self.highs.push(bar.high);
        self.lows.push(bar.low);
        self.closes.push(bar.close);
        self.volumes.push(bar.volume);

        return self;
    }
}

