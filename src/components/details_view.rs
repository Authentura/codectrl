use crate::app::AppState;
use egui::CtxRef;

#[allow(clippy::too_many_lines)]
pub fn details_view(app_state: &mut AppState, ctx: &CtxRef) {
    egui::TopBottomPanel::bottom("log_information")
        .resizable(true)
        .default_height(350.0)
        .max_height(450.0)
        .min_height(250.0)
        .show(ctx, |ui| {
            app_state.preview_height = ui.available_height() + 2.0;

            ui.horizontal(|ui| {
                ui.heading("Details");
                ui.add_space(ui.available_width() / 2.0 - 20.0);

                ui.heading("Code");

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    // u1f5d9 = 🗙
                    if ui.button("\u{1f5d9} Close").clicked() {
                        app_state.clicked_item = None;
                    }
                });
            });

            egui::Grid::new("log_information_grid")
                .num_columns(2)
                .max_col_width(ui.available_width() / 2.0)
                .min_col_width(ui.available_width() / 2.0)
                .min_row_height(ui.available_height())
                .show(ui, |ui| {
                    if let Some((log, time)) = app_state.clicked_item.as_ref() {
                        egui::ScrollArea::vertical()
                            .id_source("detail_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.add(egui::Label::new("Position:").strong());

                                        ui.label(format!(
                                            "{}:{}",
                                            &log.file_name, log.line_number
                                        ));
                                    });

                                    ui.horizontal_wrapped(|ui| {
                                        ui.add(egui::Label::new("Message:").strong());

                                        ui.label(log.message.replace("\"", ""));
                                    });

                                    ui.horizontal_wrapped(|ui| {
                                        ui.add(
                                            egui::Label::new("Message type:").strong(),
                                        );

                                        ui.label(&log.message_type);
                                    });

                                    ui.horizontal_wrapped(|ui| {
                                        ui.add(egui::Label::new("Received at:").strong());

                                        ui.label(&time.format("%F %X"));
                                    });

                                    ui.horizontal_wrapped(|ui| {
                                        ui.add(
                                            egui::Label::new("Received from:").strong(),
                                        );

                                        ui.label(&log.address);
                                    });

                                    ui.collapsing(
                                        format!(
                                            "Stack trace ({} layer(s))",
                                            log.stack.len()
                                        ),
                                        |ui| {
                                            for (index, stack) in
                                                log.stack.iter().rev().enumerate()
                                            {
                                                ui.collapsing(
                                                    format!("Stack layer {}", index),
                                                    |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.add(
                                                                egui::Label::new(
                                                                    "Position:",
                                                                )
                                                                .strong(),
                                                            );

                                                            ui.label(format!(
                                                                "{}:{} column {}",
                                                                stack.file_path,
                                                                stack.line_number,
                                                                stack.column_number
                                                            ));
                                                        });

                                                        ui.horizontal(|ui| {
                                                            ui.add(
                                                                egui::Label::new("Code:")
                                                                    .strong(),
                                                            );

                                                            ui.add(
                                                                egui::Label::new(
                                                                    &stack.code,
                                                                )
                                                                .code(),
                                                            );
                                                        });
                                                    },
                                                );
                                            }
                                        },
                                    );

                                    if !log.warnings.is_empty() {
                                        ui.collapsing(
                                            format!("{} Warning(s)", log.warnings.len()),
                                            |ui| {
                                                for (index, warning) in
                                                    log.warnings.iter().enumerate()
                                                {
                                                    ui.label(format!(
                                                        "{}. {}",
                                                        index + 1,
                                                        warning.to_string()
                                                    ));
                                                }
                                            },
                                        );
                                    }
                                });
                            });

                        egui::ScrollArea::vertical()
                            .id_source("code_scroll")
                            .auto_shrink([false, false])
                            .max_height(ui.available_height())
                            .max_width(ui.available_width())
                            .show(ui, |ui| {
                                let code = log.code_snippet.iter().fold(
                                    String::new(),
                                    |code, (line_number, line)| {
                                        if *line_number == log.line_number {
                                            format!(
                                                "{}{:>>2}  {}\n",
                                                code, line_number, line
                                            )
                                        } else {
                                            format!(
                                                "{}{: >2}  {}\n",
                                                code, line_number, line
                                            )
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

                    ui.end_row();
                });
        });
}
