use crate::{components::DARK_HEADER_FOREGROUND_COLOUR, data::window_states::AboutState};

use egui::{RichText, Ui};

pub fn draw_tab_bar(about_state: &mut AboutState, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        if ui
            .selectable_label(
                *about_state == AboutState::About,
                RichText::new(AboutState::About.to_string())
                    .heading()
                    .color(DARK_HEADER_FOREGROUND_COLOUR),
            )
            .clicked()
        {
            *about_state = AboutState::About;
        }

        if ui
            .selectable_label(
                *about_state == AboutState::Credits,
                RichText::new(AboutState::Credits.to_string())
                    .heading()
                    .color(DARK_HEADER_FOREGROUND_COLOUR),
            )
            .clicked()
        {
            *about_state = AboutState::Credits;
        }

        if ui
            .selectable_label(
                *about_state == AboutState::License,
                RichText::new(AboutState::License.to_string())
                    .heading()
                    .color(DARK_HEADER_FOREGROUND_COLOUR),
            )
            .clicked()
        {
            *about_state = AboutState::License;
        }
    });
}
