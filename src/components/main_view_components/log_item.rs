use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{Color32, RichText, Ui};

type Received = (Log<String>, DateTime<Local>);

fn draw_hover(ui: &mut Ui) {
    ui.label("Click on the \"\u{1f50e} Examine\" button to examine a log");
}

pub fn draw_log_item(
    clicked_item: &mut Option<Received>,
    received @ (log, time): &Received,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        if let Some(clicked_item) = &clicked_item {
            let _checked = ui
                .radio(*received == *clicked_item, "")
                .on_hover_ui_at_pointer(draw_hover);
        } else {
            let _checked = ui.radio(false, "").on_hover_ui_at_pointer(draw_hover);
        }

        // u1f50e = 🔎
        if ui.button("Examine \u{1f50e}").clicked() {
            *clicked_item = Some((*received).clone());
        }

        if !log.warnings.is_empty() {
            ui.label(
                RichText::new(format!("\u{26a0} {}", log.warnings.len())) // u26a0 = ⚠
                    .color(Color32::YELLOW),
            )
            .on_hover_ui_at_pointer(|ui| {
                ui.heading("Logger generated the following warning(s)");

                ui.label("");

                for (index, warning) in log.warnings.iter().enumerate() {
                    ui.label(format!("{}. {}", index + 1, warning));
                }
            });
        }
    });

    let mut message = log.message.replace("\"", "");

    if log.message.len() > 100 {
        message.truncate(97);
        message.push_str("...");
    }

    ui.label(message).on_hover_ui_at_pointer(draw_hover);
    ui.label(&log.address).on_hover_ui_at_pointer(draw_hover);
    ui.label(&log.file_name).on_hover_ui_at_pointer(draw_hover);
    ui.label(format!("{}", log.line_number))
        .on_hover_ui_at_pointer(draw_hover);
    ui.label(time.format("%F %X").to_string())
        .on_hover_ui_at_pointer(draw_hover);
    ui.end_row();
}
