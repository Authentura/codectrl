use super::{
    settings_view_components::{draw_application_settings, draw_session_settings},
    theming::DARK_HEADER_FOREGROUND_COLOUR,
};
use crate::data::AppState;

use egui::{CtxRef, Id, RichText};

pub fn settings_view(
    AppState {
        is_settings_open,
        alert_string,
        filename_format,
        preserve_session,
        message_alerts,
        application_settings,
        ..
    }: &mut AppState,
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
                    draw_session_settings(
                        message_alerts,
                        alert_string,
                        preserve_session,
                        ui,
                    );
                });
        });
}
