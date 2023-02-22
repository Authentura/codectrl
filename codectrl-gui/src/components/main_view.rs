// region: imports

use super::{main_view_components::draw_log_item, regex_filter};
use crate::data::{AppState, Filter, ISO_8601_TIME_FORMAT};
use authentura_egui_styling::{CODECTRL_GREEN, DARK_HEADER_FOREGROUND_COLOUR};
use chrono::{DateTime, Local};
use codectrl_protobuf_bindings::data::Log;
use egui::{Context, Direction, Layout, RichText, TextStyle, Ui};
use egui_extras::{Column, TableBuilder};
use regex::RegexBuilder;

// endregion

fn app_state_filter(
    is_case_sensitive: bool,
    is_using_regex: bool,
    search_filter: &str,
    filter_by: &Filter,
    log: &Log,
    time: &DateTime<Local>,
) -> bool {
    let string_filter = |search_filter: &str, search_string: &str| -> bool {
        if is_case_sensitive {
            log.message.contains(search_filter)
        } else if is_using_regex {
            regex_filter(search_filter, search_string, is_case_sensitive)
        } else {
            log.message
                .to_lowercase()
                .contains(&search_filter.to_lowercase())
        }
    };

    match filter_by {
        Filter::Message => string_filter(search_filter, &log.message),
        Filter::Time => time
            .format(ISO_8601_TIME_FORMAT)
            .to_string()
            .contains(search_filter),
        Filter::FileName => string_filter(search_filter, &log.file_name),
        Filter::Address => {
            let regex = if let Ok(regex) =  RegexBuilder::new(
                r#"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}"#,
            ).build() {
                regex
            } else {
                return false;
            };

            let address = log.address.split('.');
            let search_address = search_filter.split('.');
            let mut is_matching = true;
            let mut contains_glob = false;

            for (address_split, search_split) in address.zip(search_address) {
                if search_split == "*" {
                    contains_glob = true;
                    continue;
                }

                if !is_matching {
                    break;
                }

                match (address_split.parse::<u8>(), search_split.parse::<u8>()) {
                    (Ok(ap), Ok(sp)) => is_matching = ap == sp,
                    (Err(_), _) | (_, Err(_)) => return false,
                }
            }

            if !contains_glob
                && (!regex.is_match(&log.address) || !regex.is_match(search_filter))
            {
                return false;
            }

            is_matching
        },
        Filter::LineNumber => {
            let number = search_filter.parse::<u32>().unwrap_or(0);

            if number == 0 {
                return true;
            }

            log.line_number == number
        },
    }
}

pub fn main_view(app_state: &mut AppState, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let max_rect = ui.max_rect();

        ui.vertical_centered(|ui| {
            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - app_state.preview_height)
                .max_width(ui.available_width())
                .auto_shrink([true; 2])
                .scroll2([true; 2])
                .show(ui, |ui| {
                    let heading = |ui: &mut Ui, text| {
                        ui.heading(
                            RichText::new(text).color(DARK_HEADER_FOREGROUND_COLOUR),
                        );
                    };

                    let available_height = ui.available_height();

                    let mut table = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false)
                        .auto_shrink([true; 2])
                        .max_scroll_height(available_height)
                        .min_scrolled_height(0.0)
                        .cell_layout(Layout::centered_and_justified(
                            Direction::LeftToRight,
                        ))
                        .column(Column::exact(20.0))
                        .column(Column::remainder())
                        .column(Column::exact(150.0))
                        .column(Column::remainder())
                        .column(Column::exact(150.0))
                        .column(Column::auto())
                        .header(30.0, |mut header| {
                            header.col(|ui| heading(ui, ""));
                            header.col(|ui| heading(ui, "Message"));
                            header.col(|ui| heading(ui, "Host"));
                            header.col(|ui| heading(ui, "File name"));
                            header.col(|ui| heading(ui, "Line number"));
                            header.col(|ui| heading(ui, "Date & time"));
                        });

                    table.ui_mut().set_max_width(max_rect.width());

                    table.body(|mut body| {
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
                            body.row(60.0, |mut row| {
                                draw_log_item(
                                    &app_state.message_alerts,
                                    &mut app_state.clicked_item,
                                    app_state.do_scroll_to_selected_log,
                                    received,
                                    &mut row,
                                );
                            });
                        }
                    });
                });
        });
    });
}

pub fn main_view_empty(ctx: &Context, socket_address: &str) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("CodeCTRL")
                    .color(CODECTRL_GREEN)
                    .font(TextStyle::Name("Extra Large".into()).resolve(&ctx.style())),
            );
            ui.heading(RichText::new("by Authentura").italics());
            ui.add_space(ui.available_height() / 3.0);

            ui.heading(format!(
                "Send logs to {socket_address} and they will appear here."
            ));
        });
    });
}
