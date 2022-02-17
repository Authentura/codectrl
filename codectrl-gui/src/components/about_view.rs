use super::{
    about_view_components::{draw_about_body, draw_tab_bar},
    theming::DARK_HEADER_FOREGROUND_COLOUR,
};
use crate::data::AppState;

use egui::{CtxRef, Id, RichText};

pub fn about_view(
    AppState {
        about_state,
        is_about_open,
        ..
    }: &mut AppState,
    ctx: &CtxRef,
) {
    egui::Window::new(
        RichText::new(about_state.to_string()).color(DARK_HEADER_FOREGROUND_COLOUR),
    )
    .id(Id::new("about_view"))
    .resizable(false)
    .collapsible(false)
    .enabled(true)
    .open(is_about_open)
    .default_size((400.0, 580.0))
    .min_width(580.0)
    .min_height(400.0)
    .show(ctx, |ui| {
        draw_tab_bar(about_state, ui);

        ui.vertical_centered(|ui| {
            ui.separator();

            draw_about_body(about_state, ctx, ui);
        });
    });
}
