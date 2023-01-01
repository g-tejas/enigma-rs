use crate::utils;
use barter_data::model::{DataKind, MarketEvent};
use eframe::egui;
use egui::{
    plot::{BoxElem, BoxPlot, BoxSpread},
    Stroke,
};
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct Chart {
    pub ticker: String,
}

impl Default for Chart {
    fn default() -> Self {
        Self {
            ticker: "BTC-USDT".to_string(),
        }
    }
}

#[allow(unused_variables)] // delete this later
impl super::Widget for Chart {
    fn name(&self) -> &'static str {
        "Chart"
    }

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        events: &mut VecDeque<MarketEvent>,
        tx: Sender<MarketEvent>,
    ) {
        // Destructure self into fields
        let Self { ticker } = self;

        // Menu bar for configuring settings
        ui.horizontal(|ui| {
            if ui.button("Connect").clicked() {
                println!("Connected to ticker feed");
                crate::gateway::add_ohlcv(tx, "xbt-usd");
            }

            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", ticker))
                .show_ui(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(ticker).hint_text("Write something here"));
                    ui.label("BTC");
                    ui.label("ETH");
                });
            ui.separator();
        });

        // Plotting the shit
        let plot = egui::plot::Plot::new("Measurements");

        let sin: egui::plot::PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();

        // Filter the events to only get the candles
        let events = events.iter().filter(|event| match event.kind {
            DataKind::Candle(_) => true,
            _ => false,
        });

        let mut candles: Vec<BoxElem> = Vec::new();

        for (count, event) in events.enumerate() {
            let candle = utils::get_candle(event.clone()).unwrap();
            candles.push(
                BoxElem::new(
                    // candle.start_time.timestamp() as f64,
                    count as f64,
                    BoxSpread::new(
                        candle.low,
                        candle.open.min(candle.close),
                        (candle.open + candle.close) / 2.0,
                        candle.open.max(candle.close),
                        candle.high,
                    ),
                )
                .whisker_width(0.0)
                .fill(egui::Color32::GREEN)
                .stroke(Stroke::new(2.0, egui::Color32::GREEN)),
            );
        }
        let data = BoxPlot::new(candles);

        plot.show(ui, |plot_ui| {
            // plot_ui.line(egui::plot::Line::new(sin));
            plot_ui.box_plot(data);
        });
    }
}
