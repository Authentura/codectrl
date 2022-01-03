use crate::{
    app::AppState,
    components::about_view_components::{draw_about_body, draw_tab_bar},
};
use egui::{CtxRef, Id};

pub fn about_view(app_state: &mut AppState, ctx: &CtxRef) {
    egui::Window::new(app_state.about_state.to_string())
        .id(Id::new("about_view"))
        .resizable(false)
        .collapsible(false)
        .title_bar(true)
        .enabled(true)
        .default_size((400.0, 580.0))
        .min_width(580.0)
        .min_height(400.0)
        .open(&mut app_state.is_about_open)
        .show(ctx, |ui| {
            draw_tab_bar(&mut app_state.about_state, ui);

            ui.vertical_centered(|ui| {
                ui.separator();

                draw_about_body(&app_state.about_state, ui);
            });
        });
}
