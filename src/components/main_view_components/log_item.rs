use chrono::{DateTime, Local};
use code_ctrl_logger::Log;
use egui::{Color32, RichText, Ui};

type Received = (Log<String>, DateTime<Local>);

pub fn draw_log_item(
    clicked_item: &mut Option<Received>,
    received @ (log, time): &Received,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        if let Some(clicked_item) = &clicked_item {
            let _checked = ui.radio(*received == *clicked_item, "");
        } else {
            let _checked = ui.radio(false, "");
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

    ui.label(message);
    ui.label(&log.address);
    ui.label(&log.file_name);
    ui.label(format!("{}", log.line_number));
    ui.label(time.format("%F %X").to_string());
    ui.end_row();
}
