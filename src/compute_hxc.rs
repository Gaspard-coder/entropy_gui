use eframe::egui;

pub struct ComputeHxC {
    data_path: String,
    m: i32,
    t: i32,
}

impl Default for ComputeHxC {
    fn default() -> Self {
        Self {
            m: 2,
            t: 1,
            data_path: String::from(""),
        }
    }
}

impl ComputeHxC {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Entropy and Complexity Computing");

        ui.horizontal(|ui| {
            ui.label("Data Path: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.data_path).hint_text("~/project/example.mat"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Embedding Dimension: ");
            ui.add(egui::DragValue::new(&mut self.m).range(0..=100));
        });

        ui.horizontal(|ui| {
            ui.label("Embedding Time Delay: ");
            ui.add(egui::DragValue::new(&mut self.t).range(0..=100));
        });
    }
}
