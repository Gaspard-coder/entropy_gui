use crate::example::CodeExample;
use crate::page2::SecondPage;
use eframe::egui;

// `PartialEq` permet de comparer les variantes (ex: `current_page == Page::Example`),
// ce qui est requis par `ui.selectable_value`.
#[derive(PartialEq)]
enum Page {
    Example,
    Second,
}

// The host application
pub struct MyApp {
    // Store the component inside the main app state
    current_page: Page,
    example_widget: CodeExample,
    second_page: SecondPage,
}

impl MyApp {
    // eframe 0.37 convention: passing the CreationContext during startup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_page: Page::Example, //Starting page
            example_widget: CodeExample::default(),
            second_page: SecondPage::default(),
        }
    }
}

//Rending loop
//App is a trait (=interface)
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Horizontal navigation bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, Page::Example, "Page 1 : Example");
                ui.selectable_value(&mut self.current_page, Page::Second, "Page 2 : SecondPage");
            });

            ui.separator();
            // Display the page corresponding to self.current_page
            match self.current_page {
                Page::Example => self.example_widget.ui(ui),
                Page::Second => self.second_page.ui(ui),
            }
        });
    }
}
