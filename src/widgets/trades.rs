use chrono::{DateTime, Utc};
use csv;
use eframe::egui;
use egui::Ui;
use serde::Deserialize;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub id: i64,
    pub price: f64,
    pub qty: f64,
    pub quote_qty: f64,
    pub time: i64, // stored as a 13 digit epoch timestamp
    pub is_buyer_maker: bool,
}

pub struct Trades {
    pub data: VecDeque<Trade>,
    pub window_size: i64, // so we know where to cut off
}

impl Trades {
    pub fn new() -> Self {
        let data = read_trades().unwrap();
        let window_size = 100;
        Trades { data, window_size }
    }
}

pub fn read_trades() -> Result<VecDeque<Trade>, Box<dyn Error>> {
    let file = File::open("data/trades_min.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut trades = VecDeque::new();

    for result in reader.deserialize() {
        let trade: Trade = result?;
        trades.push_back(trade);
    }

    Ok(trades)
}

pub fn show(ui: &mut Ui) {
    ui.separator();
    use egui_extras::{Column, TableBuilder};

    // store the data here
    let data = read_trades().unwrap();

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        //.column(Column::initial(50.0).range(40.0..=300.0).resizable(true))
        // .column(
        //     Column::initial(100.0)
        //         .at_least(40.0)
        //         .resizable(true)
        //         .clip(true),
        // )
        .column(Column::auto())
        .min_scrolled_height(0.0);
    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Exchange");
            });
            header.col(|ui| {
                ui.strong("Price");
            });
            header.col(|ui| {
                ui.strong("Size");
            });
            header.col(|ui| {
                ui.strong("Time");
            });
        })
        .body(|mut body| {
            for trade in data.iter() {
                body.row(18.0, |mut row| {
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = "coinbase pro";
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                ..Default::default()
                            },
                        );

                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = trade.price.to_string();
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: if trade.is_buyer_maker {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                },
                                // background: egui::Color32::RED,
                                // background: global::COLOR_RED_TRANSPARENT,
                                ..Default::default()
                            },
                        );

                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = trade.qty.to_string();
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                background: egui::Color32::from_rgba_unmultiplied(
                                    if trade.is_buyer_maker { 255 } else { 0 },
                                    if trade.is_buyer_maker { 0 } else { 255 },
                                    0,
                                    ((trade.qty * 125.0) as i32).try_into().unwrap(),
                                ),
                                ..Default::default()
                            },
                        );
                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        // ui.label(RichText::new("ProggyClean").)
                        let now: DateTime<Utc> = Utc::now();
                        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                        ui.monospace(timestamp);
                    });
                });
            }
        });
}
