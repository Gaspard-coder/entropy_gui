use crate::example::CodeExample;
use eframe::egui;

// The host application
pub struct MyApp {
    // Store the component inside the main app state
    example_widget: CodeExample,
}

impl MyApp {
    // eframe 0.37 convention: passing the CreationContext during startup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // You can load fonts or images using _cc.egui_ctx here later
        Self {
            example_widget: CodeExample::default(),
        }
    }
}

//Rending loop
//App is a trait (=interface)
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // In eframe 0.37, CentralPanel::show takes `ui` directly instead of `ctx`
        egui::CentralPanel::default().show(ui, |ui| {
            self.example_widget.ui(ui);
        });
    }
}
