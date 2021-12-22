use crate::{
    app::{AppState, Filter},
    components::{main_view_components::draw_log_item, regex_filter},
};
use chrono::{DateTime, Local};
use code_ctrl_logger::Log;
use egui::CtxRef;

fn app_state_filter(
    is_case_sensitive: bool,
    is_using_regex: bool,
    search_filter: &str,
    filter_by: &Filter,
    log: &Log<String>,
    time: &DateTime<Local>,
) -> bool {
    match filter_by {
        Filter::Message =>
            if is_case_sensitive {
                log.message.contains(search_filter)
            } else if is_using_regex {
                regex_filter(search_filter, &log.message, is_case_sensitive)
            } else {
                log.message
                    .to_lowercase()
                    .contains(&*search_filter.to_lowercase())
            },
        Filter::Time => time.format("%F %X").to_string().contains(&*search_filter),
        Filter::FileName =>
            if is_case_sensitive {
                log.file_name.contains(&*search_filter)
            } else if is_using_regex {
                regex_filter(search_filter, &log.file_name, is_case_sensitive)
            } else {
                log.message
                    .to_lowercase()
                    .contains(&*search_filter.to_lowercase())
            },
        Filter::Address => false, // TODO: do custom ip address/host filter
        Filter::LineNumber => {
            let number = search_filter.parse::<u32>().unwrap_or(0);

            if number == 0 {
                return true;
            }

            log.line_number == number
        },
    }
}

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

                            for received in received_vec.iter().filter(|(log, time)| {
                                app_state_filter(
                                    app_state.is_case_sensitive,
                                    app_state.is_using_regex,
                                    &app_state.search_filter,
                                    &app_state.filter_by,
                                    log,
                                    time,
                                )
                            }) {
                                draw_log_item(&mut app_state.clicked_item, received, ui);
                            }
                        });
                });
        });
    });
}
