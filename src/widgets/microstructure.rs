use crate::defines::*;
use barter_data::model::MarketEvent;
use eframe::egui::plot::{LinkedAxisGroup, PlotPoint, PlotPoints};
use eframe::egui::{
    self,
    plot::{Bar, BarChart, Line, Plot},
    Color32, Ui,
};
use egui_notify::Toasts;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct MicrostructureBrowser {
    pub ticker: String,
    pub group: LinkedAxisGroup,
}

impl Default for MicrostructureBrowser {
    fn default() -> Self {
        Self {
            ticker: String::from("BTC-USDT"),
            group: LinkedAxisGroup::new(true, false),
        }
    }
}

impl super::Widget for MicrostructureBrowser {
    fn name(&self) -> &'static str {
        MICROSTRUCTURE_TITLE
    }

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        tx: Sender<MarketEvent>,
        events_tx: Sender<SysEvent>,
        _trades: &mut VecDeque<Trade>,
        _candles: &mut VecDeque<Candle>,
        best_bids: &mut VecDeque<PlotPoint>,
        best_asks: &mut VecDeque<PlotPoint>,
        _liquidations: &mut VecDeque<Liquidation>,
    ) {
        // Menu bar for configuring settings
        ui.horizontal(|ui| {
            if ui.button("Connect").clicked() {
                println!("Connected to ticker feed");
                crate::gateway::add_orderbook(tx, "BTC-USDT");
            }
        });

        // Plots
        let micro_plot = Plot::new(MICROSTRUCTURE_TITLE);
        let inv_plot = Plot::new("Inventory");

        let bb_px: PlotPoints = PlotPoints::Owned(Vec::from_iter(best_bids.iter().copied()));
        let ba_px: PlotPoints = PlotPoints::Owned(Vec::from_iter(best_asks.iter().copied()));

        let mut bar_chart = BarChart::new(vec![
            Bar::new(0.5, 1.0).name("Day 1"),
            Bar::new(1.5, 3.0).name("Day 2"),
            Bar::new(2.5, 1.0).name("Day 3"),
            Bar::new(3.5, 2.0).name("Day 4"),
            Bar::new(4.5, 4.0).name("Day 5"),
        ]);
        micro_plot
            //.link_axis(self.group.clone())
            .show_axes([false, false])
            .auto_bounds_y()
            .height(ui.available_height() * 0.8)
            .show(ui, |plot_ui| {
                plot_ui.line(egui::plot::Line::new(bb_px).color(Color32::GREEN));
                plot_ui.line(egui::plot::Line::new(ba_px).color(Color32::RED).fill(0.));
            });

        inv_plot
            .link_axis(self.group.clone())
            .show(ui, |plot_ui| {});

        //plot.height(ui.available_height() * 0.8)
        //    .show_axes([false, false])
        //    .auto_bounds_x()
        //    .link_axis(self.group.clone())
        //    .show(ui, |plot_ui| {
        //        plot_ui.line(egui::plot::Line::new(best_bid_line).width(100.));
        //        plot_ui.line(egui::plot::Line::new(best_ask_line).color(egui::Color32::RED));
        //    });
        //
        //inventory_plot
        //    .show_axes([false, false])
        //    .auto_bounds_x()
        //    .auto_bounds_y()
        //    .link_axis(self.group.clone())
        //    .show(ui, |plot_ui| {
        //        plot_ui.line(egui::plot::Line::new(PlotPoints::from_explicit_callback(
        //            move |x| x.sin(),
        //            ..,
        //            100,
        //        )));
        //    });

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

    fn settings(
        &mut self,
        ui: &mut egui::Ui,
        tx: Sender<MarketEvent>,
        events_tx: Sender<SysEvent>,
    ) {
        todo!()
    }

    fn context_menu(&self, ui: &mut Ui) {
        ui.label("Show shit");
    }
}
