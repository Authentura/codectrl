use crate::app::AboutState;
use egui::{TextStyle, Ui};

pub fn draw_tab_bar(about_state: &mut AboutState, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        if ui
            .add(
                egui::SelectableLabel::new(*about_state == AboutState::About, "About")
                    .text_style(TextStyle::Heading),
            )
            .clicked()
        {
            *about_state = AboutState::About;
        }

        if ui
            .add(
                egui::SelectableLabel::new(
                    *about_state == AboutState::Credits,
                    "Credits",
                )
                .text_style(TextStyle::Heading),
            )
            .clicked()
        {
            *about_state = AboutState::Credits;
        }

        if ui
            .add(
                egui::SelectableLabel::new(
                    *about_state == AboutState::License,
                    "License",
                )
                .text_style(TextStyle::Heading),
            )
            .clicked()
        {
            *about_state = AboutState::License;
        }
    });
}
