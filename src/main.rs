use app::Machine;
use eframe::{run_native, NativeOptions};
use std::time::Duration;
use tokio::runtime::Runtime;
mod app;
mod defines;
mod gateway;
mod utils;
mod widgets;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");
    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // This block is necessary to keep the runtime running
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let native_options = NativeOptions::default();
    run_native(
        defines::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(Machine::new(cc))),
    )
}
