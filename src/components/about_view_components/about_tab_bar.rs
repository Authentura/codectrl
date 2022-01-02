use crate::app::AboutState;
use egui::{RichText, Ui};

pub fn draw_tab_bar(about_state: &mut AboutState, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        if ui
            .selectable_label(
                *about_state == AboutState::About,
                RichText::new("About").heading(),
            )
            .clicked()
        {
            *about_state = AboutState::About;
        }

        if ui
            .selectable_label(
                *about_state == AboutState::Credits,
                RichText::new("Credits").heading(),
            )
            .clicked()
        {
            *about_state = AboutState::Credits;
        }

        if ui
            .selectable_label(
                *about_state == AboutState::License,
                RichText::new("License").heading(),
            )
            .clicked()
        {
            *about_state = AboutState::License;
        }
    });
}
