use barter_data::model::PublicTrade;
use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use eframe::egui;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use serde::Deserialize;
use std::collections::VecDeque;
use std::f64::consts::PI;

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub id: i64,
    pub price: f64,
    pub qty: f64,
    pub quote_qty: f64,
    pub time: i64, // stored as a 13 digit epoch timestamp
    pub is_buyer_maker: bool,
}

pub type Trades = VecDeque<PublicTrade>;

pub fn show(ui: &mut Ui, trade_data: &mut Trades) {
    ui.separator();

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
                ui.strong("Quantity");
            });
            header.col(|ui| {
                ui.strong("Time");
            });
        })
        .body(|mut body| {
            for trade in trade_data.iter() {
                body.row(18.0, |mut row| {
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = "binance_futures_usd"; // need to change this to get from MarketEvent
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
                                color: match trade.side {
                                    Side::Buy => egui::Color32::RED,
                                    Side::Sell => egui::Color32::GREEN,
                                },
                                // color: if trade.is_buyer_maker {
                                //     egui::Color32::RED
                                // } else {
                                //     egui::Color32::GREEN
                                // },
                                // background: egui::Color32::RED,
                                // background: global::COLOR_RED_TRANSPARENT,
                                ..Default::default()
                            },
                        );

                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = trade.quantity.to_string();
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                background: egui::Color32::from_rgba_unmultiplied(
                                    if trade.side == Side::Buy { 255 } else { 0 },
                                    if trade.side == Side::Buy { 0 } else { 255 },
                                    0,
                                    (255.0
                                        * ((((trade.quantity / 100.) * 10.).atan() + PI / 2.) / PI))
                                        .round() as u8,
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
