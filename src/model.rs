use chrono::{NaiveDate};
use std::collections::HashMap;
use serde::{Deserialize};

pub const EVENT_TYPE_MARKET: &str = "MARKET_EVENT";
const EVENT_TYPE_SIGNAL: &str = "SIGNAL_EVENT";
const EVENT_TYPE_ORDER: &str = "ORDER_EVENT";
const EVENT_TYPE_FILL: &str = "FILL_EVENT";

pub struct MarketEvent {
    pub event_type: &'static str,
    pub trace_id: String,
    pub timestamp: NaiveDate,
    pub symbol: String,
    pub close: f64,
}

// Bar represents a symbol's market data state at a fixed interval of time
#[derive(Debug, Deserialize)]
pub struct Bar {
    #[serde(rename = "Date")]
    pub timestamp: NaiveDate,
    #[serde(rename = "Open")]
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Adj Close")]
    pub close: f64,
    pub volume: f64,
}

// SymbolData represents a symbol's struct of market data arrays (OHLCV) and associated indicators values
#[derive(Debug)]
pub struct SymbolData {
    pub timestamps: Vec<NaiveDate>,
    pub opens: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<f64>,
    pub indicators: HashMap<String, Vec<f64>>,
}

impl Default for SymbolData {
    fn default() -> SymbolData {
        SymbolData{
            timestamps: vec![], opens: vec![], highs: vec![], lows: vec![],
            closes: vec![], volumes: vec![], indicators: Default::default()}
    }
}

impl SymbolData {
    pub fn add_bar(&mut self, bar: &Bar) -> &mut SymbolData {
        self.timestamps.push(bar.timestamp);
        self.opens.push(bar.open);
        self.highs.push(bar.high);
        self.lows.push(bar.low);
        self.closes.push(bar.close);
        self.volumes.push(bar.volume);

        return self;
    }
}

