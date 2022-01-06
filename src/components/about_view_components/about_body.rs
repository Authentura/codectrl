use crate::{app::AboutState, components::DARK_HEADER_FOREGROUND_COLOUR, consts};
use clap::{crate_authors, crate_description, crate_version};
use egui::{RichText, TextStyle, Ui};

pub fn draw_about_body(about_state: &AboutState, ui: &mut Ui) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| match about_state {
            AboutState::About => {
                ui.spacing();
                ui.heading("");
                ui.heading(format!(
                    "Version {} built at {}",
                    crate_version!(),
                    consts::BUILD_TIME
                ));
                ui.heading(format!("Commit: {}", consts::GIT_COMMIT));
                ui.heading(format!("Branch: {}", consts::GIT_BRANCH));
                ui.heading("");
                ui.heading(crate_description!());
            },
            AboutState::Credits => {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.strong("Authored by");
                        ui.spacing();
                        ui.vertical(|ui| {
                            for author in crate_authors!(", ").split(", ") {
                                ui.label(author);
                            }
                        });
                    });

                    ui.spacing();

                    ui.horizontal_wrapped(|ui| {
                        ui.strong("Built with the following crates");
                        ui.spacing();
                        ui.vertical(|ui| {
                            for dep in consts::BUILD_DEPS {
                                ui.label(format!("{} {}", dep.0, dep.1));
                            }
                        });
                    });
                });
            },
            AboutState::License => {
                ui.heading(
                    RichText::new("codeCTRL License")
                        .color(DARK_HEADER_FOREGROUND_COLOUR),
                );
                ui.add(
                    egui::TextEdit::multiline(&mut include_str!("../../../LICENSE"))
                        .desired_width(ui.available_width())
                        .text_style(TextStyle::Monospace),
                );

                ui.heading(
                    RichText::new("Red Hat Mono License")
                        .color(DARK_HEADER_FOREGROUND_COLOUR),
                );
                ui.add(
                    egui::TextEdit::multiline(&mut include_str!(
                        "../../../assets/fonts/red-hat/LICENSE"
                    ))
                    .desired_width(ui.available_width())
                    .text_style(TextStyle::Monospace),
                );

                ui.heading(
                    RichText::new("Roboto License").color(DARK_HEADER_FOREGROUND_COLOUR),
                );
                ui.add(
                    egui::TextEdit::multiline(&mut include_str!(
                        "../../../assets/fonts/roboto/LICENSE"
                    ))
                    .desired_width(ui.available_width())
                    .text_style(TextStyle::Monospace),
                );
            },
        });
}
