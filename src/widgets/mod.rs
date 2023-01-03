pub mod aggr_trades;
pub mod chart;
pub mod settings;

use crate::defines::{Candle, Liquidation, Trade};
use barter_data::model::{MarketEvent, OrderBook};
use eframe::egui;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

// ----------------------------------------------------------------------------

/// All widgets have to implement this trait
pub trait Widget {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        tx: Sender<MarketEvent>,
        trades: &mut VecDeque<Trade>,
        candles: &mut VecDeque<Candle>,
        orderbooks: &mut VecDeque<OrderBook>,
        liquidations: &mut VecDeque<Liquidation>,
    );
}