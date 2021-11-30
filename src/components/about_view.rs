use crate::app::{AboutState, AppState};
use clap::{crate_authors, crate_description, crate_version};
use egui::{CtxRef, Id, TextStyle};

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
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(
                        egui::SelectableLabel::new(
                            app_state.about_state == AboutState::About,
                            "About",
                        )
                        .text_style(TextStyle::Heading),
                    )
                    .clicked()
                {
                    app_state.about_state = AboutState::About;
                }

                if ui
                    .add(
                        egui::SelectableLabel::new(
                            app_state.about_state == AboutState::Credits,
                            "Credits",
                        )
                        .text_style(TextStyle::Heading),
                    )
                    .clicked()
                {
                    app_state.about_state = AboutState::Credits;
                }

                if ui
                    .add(
                        egui::SelectableLabel::new(
                            app_state.about_state == AboutState::License,
                            "License",
                        )
                        .text_style(TextStyle::Heading),
                    )
                    .clicked()
                {
                    app_state.about_state = AboutState::License;
                }
            });

            ui.vertical_centered(|ui| {
                ui.separator();

                ui.add(egui::Label::new("codeCTRL").heading().strong().underline());

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| match app_state.about_state {
                        AboutState::About => {
                            ui.spacing();
                            ui.heading("");
                            ui.heading(format!(
                                "Version {} built at {}",
                                crate_version!(),
                                super::BUILD_TIME
                            ));
                            ui.heading(format!("Commit: {}", super::GIT_COMMIT));
                            ui.heading(format!("Branch: {}", super::GIT_BRANCH));
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
                                        for dep in super::BUILD_DEPS {
                                            ui.label(format!("{} {}", dep.0, dep.1));
                                        }
                                    });
                                });
                            });
                        },
                        AboutState::License => {
                            ui.add(
                                egui::TextEdit::multiline(&mut include_str!(
                                    "../../LICENSE"
                                ))
                                .desired_width(ui.available_width())
                                .text_style(TextStyle::Monospace),
                            );
                        },
                    });
            });
        });
}
