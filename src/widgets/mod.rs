pub mod aggr_liqs;
pub mod aggr_trades;
pub mod chart;
pub mod microstructure;
pub mod settings;

use crate::defines;
use crate::defines::{Candle, Liquidation, SysEvent, Trade};
use barter_data::model::{MarketEvent, OrderBook};
use eframe::egui;
use eframe::egui::plot::PlotPoint;
use egui_notify::Toasts;
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
        events_tx: Sender<defines::SysEvent>,
        trades: &mut VecDeque<Trade>,
        candles: &mut VecDeque<Candle>,
        //orderbooks: &mut VecDeque<OrderBook>,
        best_bids: &mut VecDeque<PlotPoint>,
        best_asks: &mut VecDeque<PlotPoint>,
        liquidations: &mut VecDeque<Liquidation>,
    );

    fn settings(&mut self, ui: &mut egui::Ui, tx: Sender<MarketEvent>, events_tx: Sender<SysEvent>);

    fn context_menu(&self, ui: &mut egui::Ui);
}
