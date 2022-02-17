use crate::components::theming::DARK_HEADER_FOREGROUND_COLOUR;

use egui::{RichText, Ui};
use std::collections::BTreeSet;

pub fn draw_session_settings(
    message_alerts: &mut BTreeSet<String>,
    alert_string: &mut String,
    preserve_session: &mut bool,
    ui: &mut Ui,
) {
    ui.heading(RichText::new("Session settings").color(DARK_HEADER_FOREGROUND_COLOUR));

    ui.add_space(10.0);

    ui.indent((), |ui| {
        ui.collapsing("Alerts", |ui| {
            if message_alerts.is_empty() {
                ui.label("None");
            } else {
                egui::Grid::new("alert_grid").show(ui, |ui| {
                    for alert in message_alerts.clone() {
                        ui.label(format!("\t\u{2022} {alert}"));

                        if ui.button("Delete").clicked() {
                            message_alerts.remove(&alert.clone());
                        }

                        ui.end_row();
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.label("New alert:");
                ui.text_edit_singleline(alert_string);

                if ui.button("+").clicked() && !alert_string.is_empty() {
                    message_alerts.insert(alert_string.clone());

                    *alert_string = "".into();
                }
            });
        });

        ui.collapsing("General", |ui| {
            ui.checkbox(preserve_session, "Preserve session for next start");
        });
    });
}
