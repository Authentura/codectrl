use crate::{app::AppState, components::details_view_components::draw_information_grid};
use egui::CtxRef;

pub fn details_view(app_state: &mut AppState, ctx: &CtxRef) {
    egui::TopBottomPanel::bottom("log_information")
        .resizable(true)
        .default_height(350.0)
        .max_height(450.0)
        .min_height(250.0)
        .show(ctx, |ui| draw_information_grid(app_state, ctx, ui));
}
