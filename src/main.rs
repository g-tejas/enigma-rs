use eframe::{run_native, NativeOptions};
mod app;
mod colors;
mod defines;
mod plot;
mod utils;
mod widgets;
use app::Machine;

fn main() {
    let native_options = NativeOptions::default();
    run_native(
        defines::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(Machine::new(cc))),
    )
}
