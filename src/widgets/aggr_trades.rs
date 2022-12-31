use crate::utils::get_trade;
use crate::widgets::Widget;
use barter_data::model::{DataKind, MarketEvent};
use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct AggrTrades {
    pub filter: i32,
}

impl Default for AggrTrades {
    fn default() -> Self {
        Self { filter: 10 }
    }
}

impl Widget for AggrTrades {
    fn name(&self) -> &'static str {
        "💸 Aggregated Trades"
    }

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        trade_data: &mut VecDeque<MarketEvent>,
        tx: Sender<MarketEvent>,
    ) {
        // Destructure the self into their fields.
        let Self { filter } = self;

        // Menu bar for configuring settings
        ui.horizontal(|ui| {
            ui.menu_button("📈 Add Ticker", |ui| {
                for exchange in vec!["Binance", "Okx", "Kraken", "Coinbase"] {
                    ui.menu_button(exchange, |ui| {
                        let mut my_bool = false;
                        // ui.add(toggle(&mut my_bool));
                        ui.checkbox(&mut my_bool, "BTC-USDT");
                    });
                }
            });
            if ui.button("Connect").clicked() {
                println!("Connected to ticker feed");
                crate::gateway::add_trades(tx, "BTC-USDT".to_string());
            }
            ui.label(format!("current filter size: {}", filter));
            ui.add(egui::Slider::new(filter, 0..=100).text("Size"));
        });

        // Table
        let events = trade_data.iter().filter(|event| match event.kind {
            DataKind::Trade(_) => true,
            _ => false,
        });

        ui.separator();
        let min = trade_data
            .iter()
            .filter_map(|event| match &event.kind {
                DataKind::Trade(trade) => Some(trade.quantity),
                _ => None,
            })
            .fold(0.0, f64::min);

        let max = trade_data
            .iter()
            .filter_map(|event| match &event.kind {
                DataKind::Trade(trade) => Some(trade.quantity),
                _ => None,
            })
            .fold(0.0, f64::max);
        let range = max - min;
        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

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
            .column(Column::remainder())
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
                for event in events {
                    let trade = get_trade(event.clone());
                    body.row(text_height, |mut row| {
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
                                        Side::Buy => egui::Color32::GREEN,
                                        Side::Sell => egui::Color32::RED,
                                    },
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
                                        if trade.side == Side::Buy { 0 } else { 255 },
                                        if trade.side == Side::Buy { 255 } else { 0 },
                                        0,
                                        (((trade.quantity - min) / range) * 200.) as u8,
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
}
