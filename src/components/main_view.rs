use crate::app::{AppState, Filter};
use egui::CtxRef;
use regex::Regex;

#[allow(clippy::too_many_lines)]
pub fn main_view(app_state: &mut AppState, ctx: &CtxRef, socket_address: &str) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(format!("Listening on: {}", socket_address));

            egui::ScrollArea::vertical()
                .max_width(ui.available_width())
                .max_height(ui.available_height() - app_state.preview_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("received_grid")
                        .striped(true)
                        .spacing((0.0, 10.0))
                        .min_col_width(ui.available_width() / 6.0)
                        .max_col_width(ui.available_width() / 6.0)
                        .show(ui, |ui| {
                            ui.heading("");
                            ui.heading("Message");
                            ui.heading("Host");
                            ui.heading("File name");
                            ui.heading("Line number");
                            ui.heading("Date & time");
                            ui.end_row();

                            let received_vec = app_state.received.read().unwrap();
                            let mut received_vec: Vec<_> = received_vec.iter().collect();

                            received_vec.sort_by(|(_, a_time), (_, b_time)| {
                                if app_state.is_newest_first {
                                    b_time.partial_cmp(a_time).unwrap()
                                } else {
                                    a_time.partial_cmp(b_time).unwrap()
                                }
                            });

                            for received @ (log, time) in
                                received_vec.iter().filter(|(log, time)| match app_state
                                    .filter_by
                                {
                                    Filter::Message =>
                                        if app_state.is_case_sensitive {
                                            log.message.contains(&app_state.search_filter)
                                        } else if app_state.is_using_regex {
                                            let regex = if app_state.is_case_sensitive {
                                                Regex::new(
                                                    &app_state
                                                        .search_filter
                                                        .to_lowercase(),
                                                )
                                            } else {
                                                Regex::new(&app_state.search_filter)
                                            };

                                            if let Ok(re) = regex {
                                                if app_state.is_case_sensitive {
                                                    re.captures_iter(&log.message)
                                                        .next()
                                                        .is_some()
                                                } else {
                                                    re.captures_iter(
                                                        &log.message.to_lowercase(),
                                                    )
                                                    .next()
                                                    .is_some()
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            log.message.to_lowercase().contains(
                                                &app_state.search_filter.to_lowercase(),
                                            )
                                        },
                                    Filter::Time => time
                                        .format("%F %X")
                                        .to_string()
                                        .contains(&app_state.search_filter),
                                    Filter::FileName =>
                                        if app_state.is_case_sensitive {
                                            log.file_name
                                                .contains(&app_state.search_filter)
                                        } else if app_state.is_using_regex {
                                            let regex = if app_state.is_case_sensitive {
                                                Regex::new(
                                                    &app_state
                                                        .search_filter
                                                        .to_lowercase(),
                                                )
                                            } else {
                                                Regex::new(&app_state.search_filter)
                                            };

                                            if let Ok(re) = regex {
                                                if app_state.is_case_sensitive {
                                                    re.captures_iter(&log.file_name)
                                                        .next()
                                                        .is_some()
                                                } else {
                                                    re.captures_iter(
                                                        &log.file_name.to_lowercase(),
                                                    )
                                                    .next()
                                                    .is_some()
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            log.message.to_lowercase().contains(
                                                &app_state.search_filter.to_lowercase(),
                                            )
                                        },
                                    Filter::Address => false,
                                    Filter::LineNumber => {
                                        let number = app_state
                                            .search_filter
                                            .parse::<u32>()
                                            .unwrap_or(0);

                                        if number == 0 {
                                            return true;
                                        }

                                        log.line_number == number
                                    },
                                })
                            {
                                ui.horizontal(|ui| {
                                    if let Some(clicked_item) = &app_state.clicked_item {
                                        let _checked =
                                            ui.radio(*received == clicked_item, "");
                                    } else {
                                        let _checked = ui.radio(false, "");
                                    }

                                    // u1f50e = 🔎
                                    if ui.button("Examine \u{1f50e}").clicked() {
                                        app_state.clicked_item =
                                            Some((*received).clone());
                                    };
                                });

                                let mut message = log.message.replace("\"", "");

                                if log.message.len() > 100 {
                                    message.truncate(97);
                                    message.push_str("...");
                                }

                                ui.label(message);
                                ui.label(&log.address);
                                ui.label(&log.file_name);
                                ui.label(&log.line_number);
                                ui.label(&time.format("%F %X"));
                                ui.end_row();
                            }
                        });
                });
        });
    });
}
