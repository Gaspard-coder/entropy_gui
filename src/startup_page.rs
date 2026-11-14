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
        let mut next_page = None;

        // Dimensions of the window
        let avail_w = ui.available_width();
        let avail_h = ui.available_height();

        // Computing ratios of objects
        let btn_width = (avail_w * 0.35).clamp(160.0, 420.0);
        let btn_height = (avail_h * 0.08).clamp(32.0, 65.0);
        let spacing = (avail_h * 0.02).clamp(8.0, 24.0);
        let title_size = (avail_w * 0.035).clamp(18.0, 36.0);

        // Dynamical centering
        let total_content_height = title_size + (spacing * 2.0) + (btn_height * 2.0);
        let top_padding = ((avail_h - total_content_height) / 2.0).max(0.0);

        ui.add_space(top_padding);

        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Welcome to PermentropUI").size(title_size));
            ui.add_space(spacing * 1.5);

            if ui
                .add_sized([btn_width, btn_height], egui::Button::new("Example"))
                .clicked()
            {
                next_page = Some(Page::Example);
            }

            ui.add_space(spacing);

            if ui
                .add_sized([btn_width, btn_height], egui::Button::new("Page 2"))
                .clicked()
            {
                next_page = Some(Page::Second);
            }
        });
        next_page
    }
}
