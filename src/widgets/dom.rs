use crate::defines::*;
use crate::defines::{Candle, Liquidation, Trade};
use barter_data::model::MarketEvent;
use eframe::egui::plot::{PlotPoint, PlotPoints};
use eframe::egui::{
    self,
    plot::{Bar, BarChart, Legend, Plot},
    Color32, Ui,
};
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct DepthOfMarket {
    pub ticker: String,
}

impl Default for DepthOfMarket {
    fn default() -> Self {
        Self {
            ticker: String::from("BTC-USDT"),
        }
    }
}

impl super::Widget for DepthOfMarket {
    fn name(&self) -> &'static str {
        DOM_TITLE
    }

    fn show(
        &mut self,
        ui: &mut Ui,
        tx: Sender<MarketEvent>,
        trades: &mut VecDeque<Trade>,
        candles: &mut VecDeque<Candle>,
        best_bids: &mut VecDeque<f32>,
        best_asks: &mut VecDeque<f32>,
        liquidations: &mut VecDeque<Liquidation>,
    ) {
        // Menu bar for configuring settings
        ui.horizontal(|ui| {
            if ui.button("Connect").clicked() {
                println!("Connected to ticker feed");
                crate::gateway::add_orderbook(tx, "BTC-USDT");
            }
        });

        let plot = egui::plot::Plot::new("Microstructure Browser");

        let best_bid_line: PlotPoints = PlotPoints::from_ys_f32(best_bids.as_slices().0);
        let best_ask_line: PlotPoints = PlotPoints::from_ys_f32(best_asks.as_slices().0);

        plot.show(ui, |plot_ui| {
            plot_ui.line(egui::plot::Line::new(best_bid_line));
            plot_ui.line(egui::plot::Line::new(best_ask_line));
        });

        //let mut chart = BarChart::new(
        //    (-395..=395)
        //        .step_by(10)
        //        .map(|x| x as f64 * 0.01)
        //        .map(|x| {
        //            (
        //                x,
        //                (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
        //            )
        //        })
        //        // The 10 factor here is purely for a nice 1:1 aspect ratio
        //        .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
        //        .collect(),
        //)
        //.color(Color32::LIGHT_BLUE)
        //.name("Normal Distribution");
        //chart = chart.horizontal();
        //
        //Plot::new("Normal Distribution Demo")
        //    .legend(Legend::default())
        //    .show_axes([false, false])
        //    .show_x(false)
        //    .show_y(false)
        //    .allow_scroll(false)
        //    .allow_drag(false)
        //    //.center_x_axis(true)
        //    .center_y_axis(true)
        //    //.clamp_grid(true)
        //    .show(ui, |plot_ui| plot_ui.bar_chart(chart));
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        todo!()
    }

    fn context_menu(&self, ui: &mut Ui) {
        ui.label("Show shit");
    }
}
