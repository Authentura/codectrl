use crate::data::{ApplicationSettings, TimeFormatString};

use authentura_egui_styling::DARK_HEADER_FOREGROUND_COLOUR;
use egui::{Button, RichText, Ui};

pub fn draw_application_settings(
    application_settings: &mut ApplicationSettings,
    filename_format: &mut String,
    ui: &mut Ui,
) {
    ui.heading(RichText::new("General Settings").color(DARK_HEADER_FOREGROUND_COLOUR));

    ui.add_space(10.0);

    ui.indent((), |ui| {
        ui.collapsing("Save settings", |ui| {
            ui.checkbox(&mut application_settings.do_autosave, "Auto save")
                .on_hover_ui_at_pointer(|ui| {
                    // TODO: (AUTOSAVE) Remove this when implemented
                    ui.label("Currently unimplemented.");
                });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Filename format:");
                ui.text_edit_singleline(&mut application_settings.filename_format);

                if ui.add(Button::new("Set as filename format")).clicked() {
                    *filename_format = application_settings.filename_format.clone();
                }
            });

            if !application_settings.filename_format.is_empty() {
                let format = TimeFormatString::new(&application_settings.filename_format);

                ui.label(format!("Preview: {format}.cdctrl"));
            }

            ui.horizontal_wrapped(|ui| {
                ui.label("Please go to the");
                ui.hyperlink_to(
                    "chrono::strftime reference",
                    "https://docs.rs/chrono/latest/chrono/format/strftime/index.html",
                );
                ui.label("for valid symbols.");
            });
        });

        // ui.collapsing("Server settings", |ui| {}); // TODO: enable editing
        // server settings (i.e: port or host).
    });
}
