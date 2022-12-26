use eframe::{run_native, NativeOptions};
// use std::sync::{Arc, Mutex};
mod app;
mod colors;
mod defines;
mod plot;
mod utils;
mod widgets;
use app::Machine;

// fn random_fn(bestbid: Arc<Mutex<f64>>) {
//     let mut counter: f64 = 0.;

//     loop {
//         *bestbid.lock().unwrap() = counter;
//         counter += 1.0;
//     }
// }

fn main() {
    // Problem: What if i have a creation context, how do i access the state then?
    let app = Machine::default();
    let bestbid = app.state.bestbid.clone();
    // IT WORKS, NEW THREAD
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        *bestbid.lock().unwrap() += 1.;
    });
    let native_options = NativeOptions::default();
    run_native(
        defines::APP_NAME,
        native_options,
        // Box::new(|cc| Box::new(Machine::new(cc))),
        Box::new(|_| Box::new(app)),
    )
}
