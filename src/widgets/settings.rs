use crate::defines::*;
use barter_data::model::{MarketEvent, OrderBook};
use eframe::egui::Ui;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct Settings {
    pub api_key: String,
    pub api_secret: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            api_secret: "".to_string(),
        }
    }
}

impl super::Widget for Settings {
    fn name(&self) -> &'static str {
        SETTINGS_TITLE
    }

    fn show(
        &mut self,
        ui: &mut Ui,
        tx: Sender<MarketEvent>,
        trades: &mut VecDeque<Trade>,
        candles: &mut VecDeque<Candle>,
        orderbooks: &mut VecDeque<OrderBook>,
        liquidations: &mut VecDeque<Liquidation>,
    ) {
        let Self {
            api_key,
            api_secret,
        } = self;
        ui.heading("Enigma Machine Configuration");
        ui.separator();

        ui.collapsing("Credentials", |ui| {
            ui.horizontal(|ui| {
                ui.monospace("API Key:");
                ui.text_edit_singleline(api_key);
            });
            ui.horizontal(|ui| {
                ui.monospace("API Secret:");
                ui.text_edit_singleline(api_secret);
            });
        });
        ui.separator();

        ui.collapsing("Aesthetics", |ui| {
            ui.label("Edit shit here");
            // ui.checkbox(&mut style.tabs_are_draggable, "Tabs are draggable");
        });
    }

    fn context_menu(&self, ui: &mut Ui) {
        ui.label("from settings");
    }
}
