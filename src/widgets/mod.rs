pub mod aggr_trades;
use barter_data::model::MarketEvent;
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
        ui: &mut eframe::egui::Ui,
        trade_data: &mut VecDeque<MarketEvent>,
        tx: Sender<MarketEvent>,
    );
}
