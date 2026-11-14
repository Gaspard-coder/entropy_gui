mod app;
mod example;
mod page2;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "blank window",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
