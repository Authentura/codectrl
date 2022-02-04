use crate::data::ApplicationSettings;

use egui::Ui;

pub fn draw_application_settings(
    application_settings: &mut ApplicationSettings,
    ui: &mut Ui,
) {
    ui.heading("General Settings");

    ui.add_space(10.0);
}
