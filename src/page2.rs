use eframe::egui;

pub struct SecondPage {
    counter: i32,
}

impl Default for SecondPage {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl SecondPage {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Second Page");
        ui.label("Param");

        if ui.button("+1").clicked() {
            self.counter += 1;
        }

        ui.label(format!("Counter : {}", self.counter));
    }
}
