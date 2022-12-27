use crate::colors;
use crate::plot::candlestick_chart;
use crate::utils;
use crate::widgets::trades::{show, Trades};
use barter_data::{
    builder::Streams,
    model::{subscription::SubKind, DataKind, MarketEvent},
    ExchangeId,
};
use barter_integration::model::InstrumentKind;
use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use egui::{CentralPanel, Id, LayerId, TopBottomPanel, Ui, WidgetText};
use egui_dock::{DockArea, Node, NodeIndex, Style, StyleBuilder, TabViewer, Tree};
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};

pub struct State {
    // HashSet, so only unique widget names are stored.
    // TODO: Add unique identifier to widget names OrderBook::spot::BTCUSD for example.
    // This is so we can have multiple orderbooks up at the same time
    pub open_tabs: HashSet<String>,
    pub style: Option<Style>,
    // pub bestbid: Arc<Mutex<f64>>,
    pub bestbid: f64,
    pub tx: Sender<MarketEvent>,
    pub rx: Receiver<MarketEvent>,
    pub trade_data: Trades,
}

impl TabViewer for State {
    type Tab = String;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Welcome" => self.candlestick_chart(ui),
            "Portfolio" => self.line_chart(ui),
            "Machine Configuration" => self.machine_config(ui),
            "Orderbook" => show(ui, &mut self.trade_data), // from trades crate
            _ => {
                ui.label(tab.as_str());
            }
        }
    }

    // when you right click a tab
    fn context_menu(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.label(tab.as_str());
        ui.label("This is a context menu");
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_str().into()
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.open_tabs.remove(tab);
        true
    }
}

impl State {
    // The only things that should be stored here are styling / open_tabs related stuff
    // since the things that can be accessed from self, are very limited. Or we can store the financial
    // data here itself
    fn candlestick_chart(&mut self, ui: &mut Ui) {
        candlestick_chart(ui);
    }

    fn line_chart(&mut self, ui: &mut Ui) {
        // ui.label(format!("BTCUSDT = {}", self.bestbid.lock().unwrap()));
        ui.label(format!("BTCUSDT = {}", self.bestbid));
        // delete later
        let mut ticker = String::new();
        ui.add(egui::TextEdit::singleline(&mut ticker).hint_text("Write something here"));

        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", ticker))
            .show_ui(ui, |ui| {
                ui.label("BTC");
                ui.label("ETH");
            });

        if ui.button("Connect").clicked() {
            // match ticker {
            //     Ticker::BTC => barter(self.tx.clone(), "btc".to_string()),
            //     Ticker::ETH => barter(self.tx.clone(), "eth".to_string()),
            // }
            barter(self.tx.clone(), "eth".to_string());
        }
        let plot = Plot::new("Measurements");
        let sin: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();
        plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(sin));
        });
    }

    fn machine_config(&mut self, ui: &mut Ui) {
        ui.heading("Machine Configuration");
        // let style = self.style.as_mut().unwrap();

        ui.collapsing("Aesthetics", |ui| {
            ui.separator();
            ui.label("Edit shit here");
            // ui.checkbox(&mut style.tabs_are_draggable, "Tabs are draggable");
        });
    }
}

pub struct Machine {
    pub state: State,
    pub tree: Tree<String>,
    // pub timeseries: Arc<Mutex<VecDeque<[f64; 2]>>>,
    // pub timeseries: Arc<Mutex<f64>>,
    // pub data: FinancialData,
}

impl Machine {
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

impl Default for Machine {
    // Default Layout
    fn default() -> Self {
        let mut tree = Tree::new(vec![
            "Welcome".to_owned(),
            "Machine Configuration".to_owned(),
        ]);
        let [a, _b] = tree.split_left(NodeIndex::root(), 0.4, vec!["Orderbook".to_owned()]);
        let [_, _] = tree.split_below(a, 0.5, vec!["Portfolio".to_owned()]);
        let mut open_tabs = HashSet::new();
        for node in tree.iter() {
            if let Node::Leaf { tabs, .. } = node {
                for tab in tabs {
                    open_tabs.insert(tab.clone());
                }
            }
        }
        // let bestbid = Arc::new(Mutex::new(0.));
        let bestbid = 0.;
        let (tx, rx) = std::sync::mpsc::channel();
        let trade_data = Trades::new();
        let state = State {
            open_tabs,
            style: None,
            bestbid,
            tx,
            rx,
            trade_data,
        };
        Self { state, tree }
    }
}

impl eframe::App for Machine {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Here's where we receive data from transmitter
        if let Ok(event) = self.state.rx.try_recv() {
            match event.kind {
                DataKind::Trade(trade) => {
                    println!("{:?}", trade);
                    self.state.trade_data.push_front(trade);
                    self.state.trade_data.truncate(50); // works!
                }
                _ => println!("Failed"),
            }
            // self.state.bestbid = bb;
        }
        TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
                ui.menu_button("⚙", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                ui.menu_button("Widgets", |ui| {
                    // allow certain tabs to be toggled
                    for tab in &["Welcome", "Portfolio"] {
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
                })
            })
        });

        let panel_config = egui::containers::Frame {
            fill: colors::COLOR_BLACK,
            ..Default::default()
        };

        // Add the "workspaces feature here" > more deets in the README.md
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .frame(panel_config)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // let mut style = self
                    //     .state
                    //     .style
                    //     .get_or_insert(Style::from_egui(&ui.ctx().style()))
                    //     .clone();
                    // ui.checkbox(&mut self.state.style.tabs_are_draggable, "Lock");
                    use egui::special_emojis::{GITHUB, TWITTER};
                    ui.hyperlink_to(
                        egui::RichText::new(TWITTER),
                        "https://twitter.com/ernerfeldt",
                    );
                    ui.hyperlink_to(egui::RichText::new(GITHUB), "https://github.com/emilk/egui");
                });
            });

        CentralPanel::default().show(ctx, |_ui| {
            let layer_id = LayerId::background();
            let max_rect = ctx.available_rect();
            let clip_rect = ctx.available_rect();
            let id = Id::new("egui_dock::DockArea");
            let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);

            DockArea::new(&mut self.tree)
                .style(
                    StyleBuilder::from_egui(ctx.style().as_ref())
                        // .show_add_buttons(true)
                        // .show_add_popup(true)
                        .expand_tabs(true)
                        .build(),
                )
                .show_inside(&mut ui, &mut self.state);
        });
        ctx.request_repaint();
    }
}

// delete later
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
                // match event.kind {
                //     DataKind::Trade(trade) => {
                //         // *bestbid.lock().unwrap() = trade.price;
                //         println!("{:?}", trade);
                //         let _ = tx.send(trade);
                //     }
                //     _ => println!("Failed"),
                // }
            }
        }
    });
}
