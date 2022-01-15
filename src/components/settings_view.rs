use crate::{components::theming::DARK_HEADER_FOREGROUND_COLOUR, session::Session};
use egui::{CtxRef, Id, RichText};

pub fn settings_view(
    session: &mut Session,
    is_settings_open: &mut bool,
    alert_string: &mut String,
    ctx: &CtxRef,
) {
    egui::Window::new(RichText::new("Settings").color(DARK_HEADER_FOREGROUND_COLOUR))
        .id(Id::new("settings_view"))
        .open(is_settings_open)
        .collapsible(false)
        .resizable(true)
        .default_size((400.0, 580.0))
        .min_width(400.0)
        .min_height(400.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([true, false])
                .max_width(ui.available_width())
                .show(ui, |ui| {
                    ui.heading(
                        RichText::new("Session settings")
                            .color(DARK_HEADER_FOREGROUND_COLOUR),
                    );

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Session name:");
                        ui.text_edit_singleline(&mut session.session_name);
                    });

                    ui.label("Alerts:");

                    ui.indent("alerts", |ui| {
                        if session.message_alerts.is_empty() {
                            ui.label("None");
                        } else {
                            egui::Grid::new("alert_grid").show(ui, |ui| {
                                for alert in session.message_alerts.clone() {
                                    ui.label(&alert);

                                    if ui.button("Delete").clicked() {
                                        session.message_alerts.remove(&alert.clone());
                                    }

                                    ui.end_row();
                                }
                            });
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("New alert:");
                        ui.text_edit_singleline(alert_string);

                        if ui.button("+").clicked() && !alert_string.is_empty() {
                            session.message_alerts.insert(alert_string.clone());

                            *alert_string = "".into();
                        }
                    });
                });
        });
}
