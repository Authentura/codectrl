use chrono::{DateTime, Local};
use codectrl_protobuf_bindings::data::Log;
use egui::{Align, Color32, Label, RichText, Sense, Ui};
use egui_extras::TableRow;
use std::collections::BTreeSet;

type Received = (Log, DateTime<Local>);

fn draw_hover(ui: &mut Ui) { ui.label("Click to view log"); }

fn draw_warnings(ui: &mut Ui, log: &Log) {
    ui.heading("Logger generated the following warning(s)");
    ui.label("");

    for (index, warning) in log.warnings.iter().enumerate() {
        ui.label(format!("{index}. {warning}", index = index + 1));
    }
}

fn draw_alerts(ui: &mut Ui, exact: &str, contains: &[&String]) {
    if !exact.is_empty() {
        ui.heading("Message exactly matches the following alert");
        ui.label("");
        ui.label(exact);
    } else if !contains.is_empty() {
        ui.heading("Message contains the following alert word(s)");
        ui.label("");

        for (index, alert) in contains.iter().enumerate() {
            ui.label(format!("{index}. {alert}", index = index + 1));
        }
    }
}

pub fn draw_log_item(
    message_alerts: &BTreeSet<String>,
    clicked_item: &mut Option<Received>,
    do_scroll_to_selected_log: bool,
    received @ (log, time): &Received,
    row: &mut TableRow,
) {
    let mut message = log.message.replace('\"', "");

    let mut contains_alerts = vec![];
    let mut exact_alert = String::new();

    let mut contains_newlines = false;

    for alert in message_alerts {
        if message == *alert {
            exact_alert = message.clone();
        } else if message.contains(alert) {
            contains_alerts.push(alert);
        }
    }

    if log.message.contains('\n') {
        message = "Message contains newlines...".to_string();
        contains_newlines = true;
    } else if log.message.len() > 100 {
        message.truncate(97);
        message.push_str("...");
    }

    let labels = vec![
        if contains_newlines {
            Label::new(message)
        } else {
            Label::new(RichText::new(message).monospace())
        },
        Label::new(RichText::new(&log.address).monospace()),
        if log.file_name == "<None>" {
            Label::new(&log.file_name)
        } else {
            Label::new(RichText::new(&log.file_name).monospace())
        },
        Label::new(RichText::new(format!("{}", log.line_number)).monospace()),
        Label::new(time.format("%F %X").to_string()),
    ];

    let mut responses = vec![];

    responses.push(
        row.col(|ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(4.0);
                if let Some(clicked_item) = &clicked_item {
                    ui.radio(*received == *clicked_item, "")
                } else {
                    ui.radio(false, "")
                };

                if !log.warnings.is_empty() {
                    ui.label(
                        RichText::new(format!("\u{26a0} {}", log.warnings.len())) // u26a0 = ⚠
                            .color(Color32::YELLOW),
                    );
                }

                if !exact_alert.is_empty() {
                    ui.label(RichText::new("!!").color(Color32::RED));
                } else if !contains_alerts.is_empty() {
                    ui.label(
                        RichText::new(format!(
                            "\u{2757} {alert_count}",
                            alert_count = contains_alerts.len()
                        ))
                        .color(Color32::RED),
                    );
                }
            });
        })
        .interact(Sense::click()),
    );

    for label in labels {
        responses.push(
            row.col(|ui| {
                ui.add(label.wrap(true));
            })
            .interact(Sense::click()),
        );
    }

    let mut response =
        responses
            .iter()
            .fold(responses[0].clone(), |mut overall, current| {
                overall |= current.clone();
                overall
            });

    if let Some(clicked_item) = clicked_item {
        if do_scroll_to_selected_log && *clicked_item == *received {
            response.scroll_to_me(Some(Align::Center));
        }
    }

    response |= response.clone().on_hover_ui_at_pointer(draw_hover);

    if !log.warnings.is_empty() {
        response |= response
            .clone()
            .on_hover_ui_at_pointer(|ui| draw_warnings(ui, log));
    }

    if !exact_alert.is_empty() || !contains_alerts.is_empty() {
        response |= response
            .clone()
            .on_hover_ui_at_pointer(|ui| draw_alerts(ui, &exact_alert, &contains_alerts));
    }

    if response.clicked() {
        *clicked_item = Some((*received).clone());
    }
}
