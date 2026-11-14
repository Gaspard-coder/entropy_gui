use eframe::egui;

pub struct CodeExample {
    name: String,
    age: u32,
}

impl Default for CodeExample {
    fn default() -> Self {
        Self {
            name: String::from("Arthur"),
            age: 48,
        }
    }
}

// Demo snippet
impl CodeExample {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Saves us from writing `&mut self.name` etc
        let Self { name, age } = self;

        ui.heading("Example");
        ui.add(egui::TextEdit::singleline(name).hint_text("Name"));
        ui.add(egui::DragValue::new(age).range(0..=120).suffix(" years"));
        if ui.button("Increment").clicked() {
            *age += 1;
        }

        ui.label(format!("{name} is {age}"));
    }
}
