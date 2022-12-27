mod app;
mod colors;
mod defines;
mod plot;
mod utils;
mod widgets;
use app::Machine;
use eframe::{run_native, NativeOptions};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");
    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Problem: What if i have a creation context, how do i access the state then?
    // Do we even need this
    // std::thread::spawn(move || {
    //     rt.block_on(async {
    //         loop {
    //             tokio::time::sleep(Duration::from_secs(3600)).await;
    //         }
    //     })
    // });

    let native_options = NativeOptions::default();
    run_native(
        defines::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(Machine::new(cc))),
    )
}
