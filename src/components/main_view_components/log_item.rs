use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{Color32, Label, RichText, Sense, Ui};

type Received = (Log<String>, DateTime<Local>);

fn draw_hover(ui: &mut Ui) { ui.label("Click to view log"); }

pub fn draw_log_item(
    clicked_item: &mut Option<Received>,
    received @ (log, time): &Received,
    ui: &mut Ui,
) {
    let radio_response = ui
        .horizontal(|ui| {
            let response = if let Some(clicked_item) = &clicked_item {
                ui.radio(*received == *clicked_item, "")
            } else {
                ui.radio(false, "")
            };

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

            response
        })
        .inner;

    let mut message = log.message.replace("\"", "");

    if log.message.len() > 100 {
        message.truncate(97);
        message.push_str("...");
    }

    let labels = vec![
        Label::new(message),
        Label::new(&log.address),
        Label::new(&log.file_name),
        Label::new(format!("{}", log.line_number)),
        Label::new(time.format("%F %X").to_string()),
    ];

    let mut responses = vec![];

    for label in labels {
        responses.push(ui.add(label.sense(Sense::click())));
    }

    let mut response =
        responses
            .iter()
            .fold(responses[0].clone(), |mut overall, current| {
                overall |= current.clone();
                overall
            });

    response |= radio_response;

    ui.end_row();

    if response.on_hover_ui_at_pointer(draw_hover).clicked() {
        *clicked_item = Some((*received).clone());
    }
}
