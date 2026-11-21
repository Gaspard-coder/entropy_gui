mod app;
mod compute_hxc;
mod example;
mod page2;
mod startup_page;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "EntropUI",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
