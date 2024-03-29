use crate::defines::*;
use crate::{
    defines, utils,
    widgets::{self, Widget},
};
use barter_data::model::{DataKind, MarketEvent};
use eframe::egui;
use eframe::egui::plot::PlotPoint;
use egui_notify::Toasts;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

// TODO: Add unique identifier to widget names OrderBook::spot::BTCUSD for example.
// This is so we can have multiple orderbooks up at the same time
#[allow(dead_code)]
pub struct State<'a> {
    // Style-related fields
    open_tabs: HashSet<String>,
    style: Option<egui_dock::Style>,
    lock_layout: bool,

    // Lock-free! Channel
    tx: Sender<MarketEvent>,
    rx: Receiver<MarketEvent>,

    // Widgets
    gizmos: HashMap<&'a str, Box<dyn Widget>>,

    // Vector of pointers to a trait value Widget, might change to Hashmap
    trades: VecDeque<Trade>,
    candles: VecDeque<Candle>,
    //orderbooks: VecDeque<OrderBook>,
    best_bids: VecDeque<PlotPoint>,
    best_asks: VecDeque<PlotPoint>,
    liquidations: VecDeque<Liquidation>,

    // Notifications
    toasts: Toasts,
    events_tx: Sender<defines::SysEvent>,
    events_rx: Receiver<defines::SysEvent>,
}

impl egui_dock::TabViewer for State<'_> {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // let final_result = self.gizmos.get_mut(tab.as_str());
        match self.gizmos.get_mut(tab.as_str()) {
            Some(widget) => widget.show(
                ui,
                self.tx.clone(),
                self.events_tx.clone(),
                &mut self.trades,
                &mut self.candles,
                &mut self.best_bids,
                &mut self.best_asks,
                &mut self.liquidations,
            ),
            _ => {
                ui.heading("NO WIDGET FOUND");
            }
        }
    }

    // when you right click a tab
    fn context_menu(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match self.gizmos.get_mut(tab.as_str()) {
            Some(widget) => widget.context_menu(ui),
            _ => {
                ui.label("No context menu found");
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.open_tabs.remove(tab);
        true
    }
}

// impl State<'_> {
//     // The only things that should be stored here are styling / open_tabs related stuff
//     // since the things that can be accessed from self, are very limited. Or we can store the financial
//     // data here itself.
// }

pub struct Machine<'a> {
    state: State<'a>,
    tree: egui_dock::Tree<String>,
    ping: i64,
}

impl Machine<'_> {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut default = Self::default();

        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                default.state = state;
            }
        }
        utils::configure_fonts(&cc.egui_ctx);
        default
    }
}

impl Default for Machine<'_> {
    // Default Layout
    fn default() -> Self {
        let mut tree = egui_dock::Tree::new(vec![
            MICROSTRUCTURE_TITLE.to_owned(),
            SETTINGS_TITLE.to_owned(),
        ]);
        let [a, b] = tree.split_left(
            egui_dock::NodeIndex::root(),
            0.2,
            vec![AGGR_TRADES_TITLE.to_owned()],
        );
        let [_, _] = tree.split_below(b, 0.5, vec![AGGR_LIQS_TITLE.to_owned()]);
        let [_, _] = tree.split_below(a, 0.5, vec![CHART_TITLE.to_owned()]);
        let mut open_tabs = HashSet::new();
        for node in tree.iter() {
            if let egui_dock::Node::Leaf { tabs, .. } = node {
                for tab in tabs {
                    open_tabs.insert(tab.clone());
                }
            }
        }

        // Create channel for different components to communicate
        let (tx, rx) = std::sync::mpsc::channel();
        let (events_tx, events_rx) = std::sync::mpsc::channel();

        // Create a Hashmap of widgets
        let aggr_trades_widget: Box<dyn Widget> =
            Box::new(widgets::aggr_trades::AggrTrades::default());
        let aggr_liqs_widget: Box<dyn Widget> =
            Box::new(widgets::aggr_liqs::AggrLiquidations::default());
        let chart_widget: Box<dyn Widget> = Box::new(widgets::chart::Chart::default());
        let settings_widget: Box<dyn Widget> = Box::new(widgets::settings::Settings::default());
        let dom_widget: Box<dyn Widget> =
            Box::new(widgets::microstructure::MicrostructureBrowser::default());

        let mut gizmos: HashMap<&str, Box<dyn Widget>> = HashMap::new();
        gizmos.insert(aggr_trades_widget.name(), aggr_trades_widget);
        gizmos.insert(chart_widget.name(), chart_widget);
        gizmos.insert(settings_widget.name(), settings_widget);
        gizmos.insert(dom_widget.name(), dom_widget);
        gizmos.insert(aggr_liqs_widget.name(), aggr_liqs_widget);

        let state = State {
            open_tabs,
            style: None,
            lock_layout: false,
            tx,
            rx,
            gizmos,
            trades: VecDeque::new(),
            candles: VecDeque::new(),
            best_bids: VecDeque::new(),
            best_asks: VecDeque::new(),
            liquidations: VecDeque::new(),
            toasts: Toasts::default(),
            events_tx,
            events_rx,
        };

        Self {
            state,
            tree,
            ping: 0,
        }
    }
}

impl eframe::App for Machine<'_> {
    // #[cfg(feature = "persistence")]
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, &self.state);
    // }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(sys_event) = self.state.events_rx.try_recv() {
            self.state
                .toasts
                .success(sys_event.message)
                .set_duration(Some(Duration::from_secs(2)))
                .set_closable(true);
        }
        self.state.toasts.show(ctx);
        // Here's where we receive data from transmitter
        if let Ok(event) = self.state.rx.try_recv() {
            // Reformat the data into a flat structure.
            let exchange_time = event.exchange_time;
            let received_time = event.received_time;
            let ping = received_time - exchange_time;
            self.ping = ping.num_milliseconds();

            let exchange = format!("{}", event.exchange);
            let ticker = format!("{}-{}", event.instrument.base, event.instrument.quote);
            let instrument_type = event.instrument.kind;

            match event.kind {
                DataKind::Trade(trade) => {
                    self.state.trades.push_front(Trade {
                        exchange_time,
                        exchange,
                        ticker,
                        instrument_type,
                        price: trade.price,
                        quantity: trade.quantity,
                        side: trade.side,
                    });
                }
                DataKind::Candle(candle) => {
                    self.state.candles.push_back(Candle {
                        exchange,
                        ticker,
                        instrument_type,
                        start_time: candle.start_time,
                        end_time: candle.end_time,
                        open: candle.open,
                        high: candle.high,
                        low: candle.low,
                        close: candle.close,
                        volume: candle.volume,
                        trade_count: candle.trade_count,
                    });
                }
                DataKind::OrderBook(orderbook) => {
                    let best_bid: f64 = orderbook.bids[0].price.clone();
                    let best_ask: f64 = orderbook.asks[0].price.clone();

                    //let min_x: f64 = (exchange_time.timestamp() - 4000) as f64;

                    self.state.best_bids.push_back(PlotPoint {
                        x: exchange_time.timestamp_millis() as f64,
                        y: best_bid,
                    });
                    self.state.best_asks.push_back(PlotPoint {
                        x: exchange_time.timestamp_millis() as f64,
                        y: best_ask,
                    });

                    //let mut i = 0;
                    //while i < self.state.best_bids.len() {
                    //    if self.state.best_bids[i][0] < min_x {
                    //        self.state.best_bids.remove(i);
                    //        self.state.best_asks.remove(i);
                    //    } else {
                    //        i += 1;
                    //    }
                    //}
                }
                DataKind::Liquidation(liquidation) => {
                    self.state.liquidations.push_front(Liquidation {
                        exchange,
                        ticker,
                        instrument_type,
                        side: liquidation.side,
                        price: liquidation.price,
                        quantity: liquidation.quantity,
                        time: liquidation.time,
                    });
                }
            };
            // self.state.events.push_front(event);
            self.state.trades.truncate(50);
        }

        // Menu bar
        egui::TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.menu_button("⚙", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                ui.menu_button("Widgets", |ui| {
                    // allow certain tabs to be toggled
                    for tab in &["Settings", "Portfolio"] {
                        if ui
                            .selectable_label(self.state.open_tabs.contains(*tab), *tab)
                            .clicked()
                        {
                            if let Some(index) = self.tree.find_tab(&tab.to_string()) {
                                self.tree.remove_tab(index);
                                self.state.open_tabs.remove(*tab);
                            } else {
                                self.tree.push_to_focused_leaf(tab.to_string());
                            }

                            ui.close_menu();
                        }
                    }
                    // Not using checkbox since we want to be able to add more than one tabs
                    // for tab in &["Welcome", "Portfolio", "Watchlist", "Depth Chart"] {
                    //     ui.checkbox(&mut self.context.open_tabs.contains(*tab), *tab);
                    //     // ui.selectable_label(self.context.open_tabs.contains(*tab), *tab);
                    // }
                    ui.label("This is where we will add the various tabs");
                });
            })
        });

        let panel_config = egui::containers::Frame {
            inner_margin: egui::style::Margin {
                left: 4.,
                right: 7.,
                top: 0.,
                bottom: 3.,
            },
            fill: egui::Color32::from_rgba_unmultiplied(25, 25, 25, 200),
            ..Default::default()
        };

        // Add the "workspaces feature here" > more details in the README.md
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(25.)
            .resizable(false)
            .frame(panel_config)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.selectable_label(self.state.lock_layout, "🔒").clicked() {
                        // let style = self.state.style.as_mut().unwrap();
                        // ui.checkbox(&mut style.tabs_are_draggable, "Lock");
                        self.state.lock_layout = !self.state.lock_layout;
                        println!("locked");
                    }
                    ui.label(format!("{} ms", self.ping));
                });
            });

        // This is where the tabs and widgets are shown
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui_dock::DockArea::new(&mut self.tree)
                .style(
                    egui_dock::StyleBuilder::from_egui(ctx.style().as_ref())
                        .show_add_buttons(true)
                        .show_add_popup(true)
                        .with_separator_color_hovered(egui::Color32::LIGHT_BLUE)
                        .with_border_color(egui::Color32::RED)
                        .expand_tabs(true)
                        .build(),
                )
                .show(ctx, &mut self.state);
        });

        // Call repaint every update
        ctx.request_repaint();
    }
}
