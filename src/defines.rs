use barter_integration::model::{InstrumentKind, Side};
use chrono::{DateTime, Utc};

// ----------------------------------------------------------------------------

// Store configuration stuff here
pub const APP_NAME: &str = "enigma machine";
// pub const APP_CONFIG_NAME: &str = "tiny_pomodoro.yaml";

pub mod fonts {
    use include_flate::flate;
    flate!(pub static FONT_PROGGY_CLEAN: [u8] from "resources/ProggyClean.ttf");
}

// ----------------------------------------------------------------------------

#[allow(dead_code)]
/// Might not need instrument type, since ticker will show that
// Implement volume function?
pub struct Trade {
    pub exchange_time: DateTime<Utc>,
    pub exchange: String,
    pub ticker: String,
    pub instrument_type: InstrumentKind,
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
}

pub struct Candle {
    // pub exchange_time: DateTime<Utc>,
    pub exchange: String,
    pub ticker: String,
    pub instrument_type: InstrumentKind,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub trade_count: u64,
}

impl Candle {
    pub fn avg_price(&self) -> f64 {
        (self.open + self.high + self.low + self.close) / 4.0
    }
}

pub struct Liquidation {
    // pub exchange_time: DateTime<Utc>, // Since we have time below
    pub exchange: String,
    pub ticker: String,
    pub instrument_type: InstrumentKind,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub time: DateTime<Utc>,
}
