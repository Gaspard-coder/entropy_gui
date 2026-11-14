use eframe::egui;
//efram is the framework
// egui is the UI part

// Component part

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
    fn ui(&mut self, ui: &mut egui::Ui) {
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

// The host application
struct MyApp {
    // Store the component inside the main app state
    example_widget: CodeExample,
}

impl MyApp {
    // eframe 0.37 convention: passing the CreationContext during startup
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "blank window",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
