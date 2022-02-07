use crate::data::ApplicationSettings;

use super::{
    settings_view_components::{draw_application_settings, draw_session_settings},
    theming::DARK_HEADER_FOREGROUND_COLOUR,
};

use egui::{CtxRef, Id, RichText};
use std::collections::BTreeSet;

pub fn settings_view(
    application_settings: &mut ApplicationSettings,
    message_alerts: &mut BTreeSet<String>,
    is_settings_open: &mut bool,
    alert_string: &mut String,
    filename_format: &mut String,
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
                    draw_application_settings(application_settings, filename_format, ui);
                    draw_session_settings(message_alerts, alert_string, ui);
                });
        });
}
