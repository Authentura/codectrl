use crate::{consts, data::window_states::AboutState};

use authentura_egui_styling::{
    get_mono_license, get_sans_license, DARK_HEADER_FOREGROUND_COLOUR,
};
use chrono::Duration;
use clap::{crate_authors, crate_description, crate_version};
use codectrl_protobuf_bindings::logs_service::ServerDetails;
use egui::{Context, CursorIcon, RichText, Sense, TextStyle, Ui};
use std::time::Duration as StdDuration;

pub fn draw_about_body(
    about_state: &AboutState,
    server_details: &Option<ServerDetails>,
    ctx: &Context,
    ui: &mut Ui,
) {
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
                ui.heading("");
                if let Some(details) = server_details {
                    ui.heading(format!(
                        "Server uptime: {} minute(s)",
                        Duration::from_std(StdDuration::from_secs(details.uptime))
                            .unwrap()
                            .num_minutes(),
                    ));
                } else {
                    ui.heading("Server uptime: Fetching...");
                }
            },
            AboutState::Credits => {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.strong("Authored by");
                        ui.spacing();
                        ui.vertical(|ui| {
                            for author in crate_authors!(", ").split(", ") {
                                if let Some((index, _)) = author.match_indices('<').next()
                                {
                                    let (name, email) = author.split_at(index);

                                    let name = name.trim();
                                    let email =
                                        email.trim().replace('<', "").replace('>', "");

                                    let author_label = egui::Label::new(
                                        RichText::new(name)
                                            .color(ctx.style().visuals.hyperlink_color),
                                    );

                                    if ui
                                        .add(author_label.sense(Sense::click()))
                                        .on_hover_cursor(CursorIcon::PointingHand)
                                        .on_hover_ui_at_pointer(|ui| {
                                            ui.label("Click to copy email address");
                                        })
                                        .clicked()
                                    {
                                        ctx.output().copied_text = email;
                                    }
                                }
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
                    egui::TextEdit::multiline(&mut include_str!("../../../../LICENSE"))
                        .desired_width(ui.available_width())
                        .font(TextStyle::Monospace),
                );

                ui.heading(
                    RichText::new("Red Hat Mono License")
                        .color(DARK_HEADER_FOREGROUND_COLOUR),
                );
                ui.add(
                    egui::TextEdit::multiline(&mut get_mono_license())
                        .desired_width(ui.available_width())
                        .font(TextStyle::Monospace),
                );

                ui.heading(
                    RichText::new("Roboto License").color(DARK_HEADER_FOREGROUND_COLOUR),
                );
                ui.add(
                    egui::TextEdit::multiline(&mut get_sans_license())
                        .desired_width(ui.available_width())
                        .font(TextStyle::Monospace),
                );
            },
        });
}
