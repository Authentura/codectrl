use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{Align, Color32, Label, RichText, Sense, Ui};
use std::collections::BTreeSet;

type Received = (Log<String>, DateTime<Local>);

fn draw_hover(ui: &mut Ui) { ui.label("Click to view log"); }

pub fn draw_log_item(
    message_alerts: &BTreeSet<String>,
    clicked_item: &mut Option<Received>,
    do_scroll_to_selected_log: bool,
    received @ (log, time): &Received,
    ui: &mut Ui,
) {
    let mut message = log.message.replace('\"', "");

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
                        ui.label(format!("{index}. {warning}", index = index + 1));
                    }
                });
            }

            let mut contains_alerts = vec![];
            let mut exact_alert = String::new();

            for alert in message_alerts {
                if message == *alert {
                    exact_alert = message.clone();
                } else if message.contains(alert) {
                    contains_alerts.push(alert);
                }
            }

            if !exact_alert.is_empty() {
                ui.label(RichText::new("!!").color(Color32::RED))
                    .on_hover_ui_at_pointer(|ui| {
                        ui.heading("Message exactly matches the following alert");
                        ui.label("");
                        ui.label(exact_alert);
                    });
            } else if !contains_alerts.is_empty() {
                ui.label(
                    RichText::new(format!(
                        "\u{2757} {alert_count}",
                        alert_count = contains_alerts.len()
                    ))
                    .color(Color32::RED),
                )
                .on_hover_ui_at_pointer(|ui| {
                    ui.heading("Message contains the following alert word(s)");
                    ui.label("");

                    for (index, alert) in contains_alerts.iter().enumerate() {
                        ui.label(format!("{index}. {alert}", index = index + 1));
                    }
                });
            }

            response
        })
        .inner;

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

    response |= radio_response.clone();

    if let Some(clicked_item) = clicked_item {
        if do_scroll_to_selected_log && *clicked_item == *received {
            radio_response.scroll_to_me(Some(Align::Center));
        }
    }

    ui.end_row();

    if response.on_hover_ui_at_pointer(draw_hover).clicked() {
        *clicked_item = Some((*received).clone());
    }
}
