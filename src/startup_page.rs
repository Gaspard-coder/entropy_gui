use crate::app::Page;
use eframe::egui;

pub struct StartUpPage;

impl Default for StartUpPage {
    fn default() -> Self {
        Self {}
    }
}

impl StartUpPage {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<Page> {
        ui.heading("Welcome to PermentropUI");

        let mut next_page = None;

        if ui
            .add_sized([200.0, 40.0], egui::Button::new("Example"))
            .clicked()
        {
            next_page = Some(Page::Example);
        }

        if ui
            .add_sized([200.0, 40.0], egui::Button::new("Page 2"))
            .clicked()
        {
            next_page = Some(Page::Second);
        }
        next_page
    }
}
