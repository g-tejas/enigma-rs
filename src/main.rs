use eframe::{run_native, NativeOptions};
mod app;
mod colors;
mod plot;
mod widgets;
use app::Machine;

fn main() {
    let native_options = NativeOptions::default();
    run_native(
        "enigma",
        native_options,
        Box::new(|cc| Box::new(Machine::new(cc))),
    )
}
