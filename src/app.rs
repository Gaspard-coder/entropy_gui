use crate::example::CodeExample;
use crate::page2::SecondPage;
use crate::startup_page::StartUpPage;
use eframe::egui;

// `PartialEq` permet de comparer les variantes (ex: `current_page == Page::Example`),
// ce qui est requis par `ui.selectable_value`.
#[derive(PartialEq)]
pub enum Page {
    StartUpPage,
    Example,
    Second,
}

// The host application
pub struct MyApp {
    // Store the component inside the main app state
    current_page: Page,
    start_up_page: StartUpPage,
    example_widget: CodeExample,
    second_page: SecondPage,
}

impl MyApp {
    // eframe 0.37 convention: passing the CreationContext during startup
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_page: Page::StartUpPage, //Starting page
            start_up_page: StartUpPage::default(),
            example_widget: CodeExample::default(),
            second_page: SecondPage::default(),
        }
    }
}

//Rending loop
//App is a trait (=interface)
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Deactivate the navigation bar if on starting page
        if self.current_page != Page::StartUpPage {
            egui::Panel::top("navbar").show(ui, |ui| {
                ui.horizontal(|ui| {
                    /*
                    if ui.button("⬅ Home").clicked() {
                        self.current_page = Page::StartUpPage;
                    }
                    */
                    ui.separator();
                    ui.selectable_value(&mut self.current_page, Page::Example, "Test 1");
                    ui.selectable_value(&mut self.current_page, Page::Second, "Test 2");
                });
            });
        }

        //Navigation from the startup page
        egui::CentralPanel::default().show(ui, |ui| match self.current_page {
            Page::StartUpPage => {
                // If a button was clicked, switch to the requested page
                if let Some(target_page) = self.start_up_page.ui(ui) {
                    self.current_page = target_page;
                }
            }
            Page::Example => self.example_widget.ui(ui),
            Page::Second => self.second_page.ui(ui),
        });
    }
}
