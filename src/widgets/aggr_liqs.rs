use crate::defines;
use crate::defines::*;
use barter_data::{
    model::{
        subscription::{SubKind, Subscription},
        MarketEvent,
    },
    ExchangeId,
};
use barter_integration::model::{Instrument, InstrumentKind, Side, Symbol};
use eframe::egui::{self, Ui};
use egui::plot::PlotPoint;
use egui_extras::{Column, TableBuilder};
use egui_notify::Toasts;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub struct AggrLiquidations {
    pub filter: i32,
    pub show_settings: bool,

    // These are the input fields
    pub exchange: ExchangeId,
    pub ticker: String,
    pub instrument_kind: InstrumentKind,

    // HashSet to contain all subscriptions so we don't repeat any.
    pub subscriptions: HashSet<Subscription>,
}

impl Default for AggrLiquidations {
    fn default() -> Self {
        Self {
            filter: 10,
            show_settings: true,
            exchange: ExchangeId::BinanceFuturesUsd,
            ticker: String::new(),
            instrument_kind: InstrumentKind::FuturePerpetual,
            subscriptions: HashSet::new(),
        }
    }
}

impl super::Widget for AggrLiquidations {
    fn name(&self) -> &'static str {
        AGGR_LIQS_TITLE
    }

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        tx: Sender<MarketEvent>,
        events_tx: Sender<defines::SysEvent>,
        trades: &mut VecDeque<Trade>,
        candles: &mut VecDeque<Candle>,
        best_bids: &mut VecDeque<PlotPoint>,
        best_asks: &mut VecDeque<PlotPoint>,
        liquidations: &mut VecDeque<Liquidation>,
    ) {
        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut self.filter, 0..=100).text("Size"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.selectable_label(self.show_settings, "🛠").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        ui.separator();

        if self.show_settings {
            self.settings(ui, tx, events_tx);
        } else {
            let min = liquidations
                .iter()
                .filter_map(|trade| Some(trade.quantity))
                .fold(0.0, f64::min);

            let max = liquidations
                .iter()
                .filter_map(|trade| Some(trade.quantity))
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
                    for liq in liquidations {
                        if liq.quantity * liq.price > (self.filter * 200).into() {
                            body.row(text_height, |mut row| {
                                row.col(|ui| {
                                    let mut layout_job = egui::text::LayoutJob::default();
                                    let text = liq.exchange.as_str(); // need to change this to get from MarketEvent
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
                                    let text = liq.price.to_string();
                                    layout_job.append(
                                        &text,
                                        0.0,
                                        egui::text::TextFormat {
                                            font_id: egui::FontId::monospace(15.0),
                                            color: match liq.side {
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
                                    let text = liq.quantity.to_string();
                                    layout_job.append(
                                        &text,
                                        0.0,
                                        egui::text::TextFormat {
                                            font_id: egui::FontId::monospace(15.0),
                                            color: egui::Color32::WHITE,
                                            background: egui::Color32::from_rgba_unmultiplied(
                                                if liq.side == Side::Buy { 0 } else { 255 },
                                                if liq.side == Side::Buy { 255 } else { 0 },
                                                0,
                                                (((liq.quantity - min) / range) * 200.) as u8,
                                            ),
                                            ..Default::default()
                                        },
                                    );
                                    ui.label(layout_job);
                                });
                                row.col(|ui| {
                                    let timestamp = liq.time.format("%H:%M:%S").to_string();
                                    ui.monospace(timestamp);
                                });
                            });
                        }
                    }
                });
        }
    }

    fn settings(
        &mut self,
        ui: &mut egui::Ui,
        tx: Sender<MarketEvent>,
        events_tx: Sender<SysEvent>,
    ) {
        ui.heading("Add feeds");
        egui::Grid::new("Settings Grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Select Exchange");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.exchange))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.exchange,
                            ExchangeId::BinanceFuturesUsd,
                            ExchangeId::BinanceFuturesUsd.as_str(),
                        );
                        ui.selectable_value(
                            &mut self.exchange,
                            ExchangeId::Binance,
                            ExchangeId::Binance.as_str(),
                        );
                        ui.selectable_value(
                            &mut self.exchange,
                            ExchangeId::Coinbase,
                            ExchangeId::Coinbase.as_str(),
                        );
                        ui.selectable_value(
                            &mut self.exchange,
                            ExchangeId::Kraken,
                            ExchangeId::Kraken.as_str(),
                        );
                    });
                ui.end_row();

                ui.label("Ticker");
                ui.add(egui::TextEdit::singleline(&mut self.ticker).hint_text("for e.g, BTC-USDT"));
                ui.end_row();

                ui.label("Select instrument type");
                egui::ComboBox::from_id_source("test")
                    .selected_text(format!("{:?}", self.instrument_kind))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.instrument_kind,
                            InstrumentKind::FuturePerpetual,
                            "Future Perpetual",
                        );
                        ui.selectable_value(
                            &mut self.instrument_kind,
                            InstrumentKind::Spot,
                            "Spot",
                        );
                    });
                ui.end_row();
            });

        // For connecting to the feeds.
        if ui.button("Add to feed").clicked() {
            if let Ok((base, quote)) = crate::utils::split_ticker(self.ticker.as_str()) {
                let new_sub = Subscription {
                    exchange: self.exchange,
                    instrument: Instrument {
                        base: Symbol::new(base),
                        quote: Symbol::new(quote),
                        kind: self.instrument_kind,
                    },
                    kind: SubKind::Liquidation,
                };
                if !self.subscriptions.contains(&new_sub) {
                    crate::gateway::add_stream(tx.clone(), new_sub.clone());
                }
                self.subscriptions.insert(new_sub);
            };
        }
        ui.separator();

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Exchange");
                });
                header.col(|ui| {
                    ui.strong("Base");
                });
                header.col(|ui| {
                    ui.strong("Quote");
                });
                header.col(|ui| {
                    ui.strong("Instrument Kind");
                });
            })
            .body(|mut body| {
                for subscription in &self.subscriptions {
                    body.row(text_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{:?}", subscription.exchange));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", subscription.instrument.base));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", subscription.instrument.quote));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", subscription.instrument.kind));
                        });
                    });
                }
            });
    }

    fn context_menu(&self, ui: &mut Ui) {
        ui.label("from aggr trades");
    }
}
