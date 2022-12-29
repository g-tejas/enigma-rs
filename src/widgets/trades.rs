use barter_data::model::{DataKind, MarketEvent};
use barter_data::{builder::Streams, model::subscription::SubKind, ExchangeId};
use barter_integration::model::InstrumentKind;
use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use eframe::egui;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use futures::StreamExt;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub type Trades = VecDeque<MarketEvent>;

pub fn show(ui: &mut Ui, trade_data: &mut Trades, tx: Sender<MarketEvent>) {
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Connect").clicked() {
            // match ticker {
            //     Ticker::BTC => barter(self.tx.clone(), "btc".to_string()),
            //     Ticker::ETH => barter(self.tx.clone(), "eth".to_string()),
            // }
            barter(tx, "eth".to_string());
        }
        if ui.button("Open window").clicked() {
            egui::Window::new("Hello").show(ui.ctx(), |ui| {
                ui.label("Hgello wrold");
            });
        }
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
                        let text = get_price(trade).to_string();
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: match get_side(trade) {
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
                        let text = get_quantity(trade).to_string();
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                background: egui::Color32::from_rgba_unmultiplied(
                                    if get_side(trade) == Side::Buy { 0 } else { 255 },
                                    if get_side(trade) == Side::Buy { 255 } else { 0 },
                                    0,
                                    (((get_quantity(trade) - min) / range) * 200.) as u8,
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

fn get_price(event: &MarketEvent) -> f64 {
    match event.kind {
        DataKind::Trade(ref trade) => trade.price,
        _ => 0.0,
    }
}

fn get_quantity(event: &MarketEvent) -> f64 {
    match event.kind {
        DataKind::Trade(ref trade) => trade.quantity,
        _ => 0.0,
    }
}

fn get_side(event: &MarketEvent) -> Side {
    match event.kind {
        DataKind::Trade(ref trade) => trade.side,
        _ => Side::Buy,
    }
}

fn barter(tx: Sender<MarketEvent>, ticker: String) {
    tokio::spawn(async move {
        loop {
            let streams = Streams::builder()
                .subscribe([(
                    ExchangeId::BinanceFuturesUsd,
                    ticker.as_str(),
                    "usdt",
                    InstrumentKind::FuturePerpetual,
                    SubKind::Trade,
                )])
                .init()
                .await
                .unwrap();
            let mut joined_stream = streams.join_map::<MarketEvent>().await;

            while let Some((_exchange, event)) = joined_stream.next().await {
                let _result = tx.send(event);
            }
        }
    });
}
