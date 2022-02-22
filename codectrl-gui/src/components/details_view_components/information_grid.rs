use crate::{
    components::{
        details_view_components::code_highlighter, message_preview_view,
        DARK_HEADER_FOREGROUND_COLOUR,
    },
    data::AppState,
};

use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{CtxRef, RichText, TextStyle, Ui};

pub fn draw_information_grid(app_state: &mut AppState, ctx: &CtxRef, ui: &mut Ui) {
    app_state.preview_height = ui.available_height() + 2.0;

    ui.horizontal(|ui| {
        let heading_width = "Log information".chars().fold(0.0, |sum, c| {
            sum + ui.fonts().glyph_width(TextStyle::Heading, c)
        });

        ui.add_space((ui.available_width() / 2.0) - heading_width * 0.5);
        ui.heading(RichText::new("Log information").color(DARK_HEADER_FOREGROUND_COLOUR));

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
            ui.heading(RichText::new("Details").color(DARK_HEADER_FOREGROUND_COLOUR));
            ui.heading(RichText::new("Code").color(DARK_HEADER_FOREGROUND_COLOUR));
        });

    egui::Grid::new("log_information_grid")
        .num_columns(2)
        .max_col_width(ui.available_width() / 2.0)
        .min_col_width(ui.available_width() / 2.0)
        .min_row_height(ui.available_height())
        .show(ui, |ui| {
            if let Some((log, time)) = app_state.clicked_item.clone() {
                detail_scroll(app_state, &log, &time, ctx, ui);
                code_scroll(
                    &mut app_state.is_copying_line_indicator,
                    &mut app_state.is_copying_line_numbers,
                    &mut app_state.copy_language,
                    &log,
                    ctx,
                    ui,
                );
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
                    ui.label("Position:");
                    ui.label(format!("{}:{}", &log.file_name, log.line_number));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Message:");

                    if log.message.len() <= 200 {
                        ui.code(log.message.replace('\"', ""));
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
                    ui.label("Message type:");
                    ui.code(&log.message_type);
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Received at:");
                    ui.label(time.format("%F %X").to_string());
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Received from:");
                    ui.label(&log.address);
                });

                ui.collapsing(
                    format!("Stacktrace ({} layer(s))", log.stack.len()),
                    |ui| {
                        if log.stack.is_empty() {
                            ui.label("No stacktrace");
                            return;
                        }

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

fn code_scroll(
    is_copying_line_numbers: &mut bool,
    is_copying_line_indicator: &mut bool,
    copy_language: &mut String,
    log: &Log<String>,
    ctx: &CtxRef,
    ui: &mut Ui,
) {
    egui::ScrollArea::vertical()
        .id_source("code_scroll")
        .auto_shrink([false, false])
        .max_height(ui.available_height())
        .max_width(ui.available_width() - 10.0)
        .show(ui, |ui| {
            let (indicated_code, code, numbered_code) = log.code_snippet.0.iter().fold(
                (String::new(), String::new(), String::new()),
                |code, (line_number, line)| {
                    if *line_number == log.line_number {
                        (
                            format!("{code}{line_number:>>3}  {line}\n", code = code.0),
                            format!("{code}{line}\n", code = code.1),
                            format!("{code}{line_number: >3}  {line}\n", code = code.2),
                        )
                    } else {
                        (
                            format!("{code}{line_number: >3}  {line}\n", code = code.0),
                            format!("{code}{line}\n", code = code.1),
                            format!("{code}{line_number: >3}  {line}\n", code = code.2),
                        )
                    }
                },
            );

            let mut indicated_code = indicated_code.trim_end().to_string();

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                // TODO: Hardcoded Langauge should be accepted from the log

                let mut layout_job: egui::text::LayoutJob =
                    code_highlighter(string, "rust"); // Change rust to log.language or something
                layout_job.wrap_width = wrap_width;
                ui.fonts().layout_job(layout_job)
            };

            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut indicated_code)
                    .desired_width(ui.available_width())
                    .interactive(false)
                    .layouter(&mut layouter)
                    .code_editor(),
            )
            .interact(egui::Sense::click())
            .on_hover_ui_at_pointer(|ui| {
                ui.label("Right-click to open context menu");
            })
            .context_menu(|ui| {
                ui.menu_button("Copy", |ui| {
                    if ui.button("Copy with line number indictator").clicked() {
                        ctx.output().copied_text = indicated_code.clone();
                    }

                    if ui.button("Copy only code").clicked() {
                        ctx.output().copied_text = code.trim_end().to_string();
                    }

                    ui.menu_button("Copy code with Markdown formatting", |ui| {
                        ui.checkbox(is_copying_line_numbers, "Copy line numbers");
                        ui.checkbox(is_copying_line_indicator, "Copy line indicator");

                        if *is_copying_line_indicator {
                            *is_copying_line_numbers = true;
                        }

                        ui.horizontal(|ui| {
                            ui.label("Language: ");
                            ui.text_edit_singleline(copy_language);
                        });

                        if ui.button("Copy with settings").clicked() {
                            let code = if *is_copying_line_indicator {
                                indicated_code
                            } else if *is_copying_line_numbers {
                                numbered_code
                            } else {
                                code
                            };

                            let formatted_code =
                                format!("```{}\n{}\n```", copy_language, code.trim_end());

                            ctx.output().copied_text = formatted_code;
                        }
                    });
                });
            });
        });
}
