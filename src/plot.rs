use eframe::{
    egui::{self},
    epaint::Color32,
    App, Frame,
};
use egui::{
    plot::{BoxElem, BoxPlot, BoxSpread, Line, Plot, PlotPoints},
    CentralPanel, Context, ScrollArea, Stroke, Ui, Window,
};
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;

use csv;

#[derive(Debug, Deserialize)]
pub struct Candlestick {
    pub open_time: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: f64,
}

impl Candlestick {
    pub fn avg_price(&self) -> f64 {
        (self.open + self.high + self.low + self.close) / 4.0
    }
}

pub struct Chart {
    pub data: Vec<Candlestick>,
}

pub fn read_candlesticks(file_path: &str) -> Result<Vec<Candlestick>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut candlesticks = Vec::new();

    for result in reader.deserialize() {
        let candlestick: Candlestick = result?;
        candlesticks.push(candlestick);
    }

    Ok(candlesticks)
}

pub fn candlestick_chart(ui: &mut Ui) {
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);
    let data = BoxPlot::new(vec![
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.5, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(2.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(2.5, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.5, BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(4.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(4.5, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(5.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(5.5, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1))
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.5, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2))
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
    ]);

    let plot = Plot::new("candlestick chart");
    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}

impl Chart {
    pub fn new() -> Chart {
        let candlesticks = read_candlesticks("btcusdt.csv").unwrap();
        Chart { data: candlesticks }
    }
}

impl App for Chart {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let ohlcv = &self.data;
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| {
                for candlestick in ohlcv.iter() {
                    ui.horizontal(|ui| {
                        ui.label(&candlestick.avg_price().to_string());
                        ui.label(&candlestick.open.to_string());
                        ui.label(&candlestick.high.to_string());
                        ui.label(&candlestick.low.to_string());
                        ui.label(&candlestick.close.to_string());
                        ui.label(&candlestick.volume.to_string());
                    });
                }
            });
        });

        Window::new("Candlestick Plot").show(ctx, |ui| {
            candlestick_chart(ui);
        });
        let plot = Plot::new("Measurements");
        let mut xy = Vec::new();
        for (i, o) in ohlcv.iter().enumerate() {
            xy.push((i, o.avg_price()));
        }
        let sin: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();
        Window::new("Random Plot").show(ctx, |ui| {
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(sin));
            });
        });
    }
}
