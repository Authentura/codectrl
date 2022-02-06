use crate::{components::DARK_HEADER_FOREGROUND_COLOUR, data::ApplicationSettings};

use egui::{Button, Color32, Label, RichText, Ui};

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
                ui.add(
                    match time_format::strftime_utc(
                        &application_settings.filename_format,
                        45296, // January 1st 1970 at 12:34:56 PM UTC.
                    ) {
                        Ok(label) => Label::new(format!("Preview: {label}.cdctrl")),
                        Err(_) => Label::new(
                            RichText::new("Unable to format string").color(Color32::RED),
                        ),
                    },
                );
            }

            ui.horizontal_wrapped(|ui| {
                ui.label("Please go to the");
                ui.hyperlink_to(
                    "strftime reference",
                    "https://www.cplusplus.com/reference/ctime/strftime/",
                );
                ui.label("for valid symbols.");
            });
        });

        // ui.collapsing("Server settings", |ui| {}); // TODO: enable editing
        // server settings (i.e: port or host).
    });
}
