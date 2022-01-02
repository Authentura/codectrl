use crate::{app::AppState, components::message_preview_view};
use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{CtxRef, RichText, Ui};

pub fn draw_information_grid(app_state: &mut AppState, ctx: &CtxRef, ui: &mut Ui) {
    app_state.preview_height = ui.available_height() + 2.0;

    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Log information")
                .heading()
                .underline()
                .strong(),
        );

        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            // u1f5d9 = 🗙
            if ui.button("\u{1f5d9} Close").clicked() {
                app_state.clicked_item = None;
            }
        });
    });

    ui.separator();
    egui::Grid::new("log_information_grid_headers")
        .num_columns(2)
        .max_col_width(ui.available_width() / 2.0)
        .min_col_width(ui.available_width() / 2.0)
        .show(ui, |ui| {
            ui.heading("Details");
            ui.heading("Code");
        });

    egui::Grid::new("log_information_grid")
        .num_columns(2)
        .max_col_width(ui.available_width() / 2.0)
        .min_col_width(ui.available_width() / 2.0)
        .min_row_height(ui.available_height())
        .show(ui, |ui| {
            if let Some((log, time)) = app_state.clicked_item.clone() {
                detail_scroll(app_state, &log, &time, ctx, ui);
                code_scroll(&log, ui);
            }

            ui.end_row();
        });
}

fn detail_scroll(
    app_state: &mut AppState,
    log: &Log<String>,
    time: &DateTime<Local>,
    ctx: &CtxRef,
    ui: &mut Ui,
) {
    egui::ScrollArea::vertical()
        .id_source("detail_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Position:").strong());

                    ui.label(format!("{}:{}", &log.file_name, log.line_number));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Message:").strong());

                    if log.message.len() <= 200 {
                        ui.label(log.message.replace("\"", ""));
                    } else {
                        // u25b6 = ▶
                        if ui.button("View message \u{25b6}").clicked() {
                            app_state.is_message_preview_open =
                                !app_state.is_message_preview_open;
                        }

                        message_preview_view(
                            &mut app_state.is_message_preview_open,
                            ctx,
                            &log.message,
                            &log.message_type,
                        );
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Message type:").strong());

                    ui.label(&log.message_type);
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Received at:").strong());

                    ui.label(RichText::new(time.format("%F %X").to_string()));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Received from:").strong());

                    ui.label(&log.address);
                });

                ui.collapsing(
                    format!("Stack trace ({} layer(s))", log.stack.len()),
                    |ui| {
                        for (index, stack) in log.stack.iter().rev().enumerate() {
                            ui.collapsing(format!("Stack layer {}", index), |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Position:").strong());

                                    ui.label(format!(
                                        "{}:{} column {}",
                                        stack.file_path,
                                        stack.line_number,
                                        stack.column_number
                                    ));
                                });

                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Code:").strong());

                                    ui.label(RichText::new(&stack.code).code());
                                });
                            });
                        }
                    },
                );

                if !log.warnings.is_empty() {
                    ui.collapsing(format!("{} Warning(s)", log.warnings.len()), |ui| {
                        for (index, warning) in log.warnings.iter().enumerate() {
                            ui.label(format!("{}. {}", index + 1, warning));
                        }
                    });
                }
            });
        });
}

fn code_scroll(log: &Log<String>, ui: &mut Ui) {
    egui::ScrollArea::vertical()
        .id_source("code_scroll")
        .auto_shrink([false, false])
        .max_height(ui.available_height())
        .max_width(ui.available_width() - 10.0)
        .show(ui, |ui| {
            let code = log.code_snippet.0.iter().fold(
                String::new(),
                |code, (line_number, line)| {
                    if *line_number == log.line_number {
                        format!("{}{:>>3}  {}\n", code, line_number, line)
                    } else {
                        format!("{}{: >3}  {}\n", code, line_number, line)
                    }
                },
            );

            let mut code = code.trim_end().to_string();

            ui.add(
                egui::TextEdit::multiline(&mut code)
                    .desired_width(ui.available_width())
                    .interactive(false)
                    .code_editor(),
            );
        });
}
