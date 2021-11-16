use crate::common::{Received, Receiver};
use chrono::{DateTime, Local};
use code_ctrl_logger::Log;
use egui::CtxRef;
use epi::{Frame, Storage};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{mpsc::Receiver as Rx, Arc, Mutex, RwLock},
    thread::{Builder as ThreadBuilder, JoinHandle},
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Filter {
    Message,
    Time,
    FileName,
    Address,
    LineNumber,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message => write!(f, "Message"),
            Self::Time => write!(f, "Time"),
            Self::FileName => write!(f, "File name"),
            Self::Address => write!(f, "Address"),
            Self::LineNumber => write!(f, "Line number"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GuiAppState {
    search_filter: String,
    filter_by: Filter,
    #[serde(skip)]
    received: Received,
    is_case_sensitive: bool,
    is_using_regex: bool,
    is_newest_first: bool,
    #[serde(skip)]
    clicked_item: Option<(Log<String>, DateTime<Local>)>,
    #[serde(skip)]
    preview_height: f32,
}

impl Default for GuiAppState {
    fn default() -> Self {
        Self {
            search_filter: "".into(),
            filter_by: Filter::Message,
            received: Arc::new(RwLock::new(VecDeque::new())),
            is_case_sensitive: false,
            is_using_regex: false,
            is_newest_first: true,
            clicked_item: None,
            preview_height: 0.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GuiApp {
    #[serde(skip)]
    pub receiver: Receiver,
    #[serde(skip)]
    update_thread: Option<JoinHandle<()>>,
    data: GuiAppState,
    title: String,
    socket_address: String,
}

impl GuiApp {
    pub fn new(title: &str, receiver: Rx<Log<String>>, socket_address: String) -> Self {
        Self {
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            data: GuiAppState::default(),
            title: title.into(),
            socket_address,
        }
    }
}

impl epi::App for GuiApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Filter: ");
                    ui.text_edit_singleline(&mut self.data.search_filter);

                    if ui.button("🗙").clicked() {
                        self.data.search_filter = "".into();
                    }

                    ui.label("Filter by:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.data.filter_by))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Message,
                                format!("{}", Filter::Message),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Time,
                                format!("{}", Filter::Time),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::FileName,
                                format!("{}", Filter::FileName),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Address,
                                format!("{}", Filter::Address),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::LineNumber,
                                format!("{}", Filter::LineNumber),
                            );
                        });

                    ui.checkbox(&mut self.data.is_case_sensitive, "Case sensitive");
                    ui.checkbox(&mut self.data.is_using_regex, "Regex");

                    ui.separator();

                    if ui
                        .button(
                            if self.data.is_newest_first {
                                "⬇ Newest first"
                            } else {
                                "⬆ Newest last"
                            },
                        )
                        .clicked()
                    {
                        self.data.is_newest_first = !self.data.is_newest_first;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("Listening on: {}", self.socket_address));

                egui::ScrollArea::vertical()
                    .max_width(ui.available_width())
                    .max_height(ui.available_height() - self.data.preview_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Grid::new("received_grid")
                            .striped(true)
                            .spacing((0.0, 10.0))
                            .min_col_width(ui.available_width() / 6.0)
                            .show(ui, |ui| {
                                ui.heading("");
                                ui.heading("Message");
                                ui.heading("Host");
                                ui.heading("File name");
                                ui.heading("Line number");
                                ui.heading("Date & time");
                                ui.end_row();

                                let received_vec = self.data.received.read().unwrap();
                                let mut received_vec: Vec<_> =
                                    received_vec.iter().collect();

                                received_vec.sort_by(|(_, a_time), (_, b_time)| {
                                    if self.data.is_newest_first {
                                        b_time.partial_cmp(a_time).unwrap()
                                    } else {
                                        a_time.partial_cmp(b_time).unwrap()
                                    }
                                });

                                for received in received_vec.iter().filter(|(log, _)| {
                                    match self.data.filter_by {
                                        Filter::Message =>
                                            if self.data.is_case_sensitive {
                                                log.message
                                                    .contains(&self.data.search_filter)
                                            } else {
                                                log.message.to_lowercase().contains(
                                                    &self
                                                        .data
                                                        .search_filter
                                                        .to_lowercase(),
                                                )
                                            },
                                        Filter::Time => todo!(),
                                        Filter::FileName =>
                                            if self.data.is_case_sensitive {
                                                log.file_name
                                                    .contains(&self.data.search_filter)
                                            } else {
                                                log.message.to_lowercase().contains(
                                                    &self
                                                        .data
                                                        .search_filter
                                                        .to_lowercase(),
                                                )
                                            },
                                        Filter::Address => todo!(),
                                        Filter::LineNumber => todo!(),
                                    }
                                }) {
                                    ui.horizontal(|ui| {
                                        if let Some(clicked_item) =
                                            &self.data.clicked_item
                                        {
                                            let _ =
                                                ui.radio(*received == clicked_item, "");
                                        } else {
                                            let _ = ui.radio(false, "");
                                        }

                                        if ui.button("Examine 🔎").clicked() {
                                            self.data.clicked_item =
                                                Some((*received).clone())
                                        };
                                    });

                                    ui.label(&received.0.message.replace("\"", ""));
                                    ui.label("Not known");
                                    ui.label(&received.0.file_name);
                                    ui.label(&received.0.line_number);
                                    ui.label(&received.1.format("%F %X"));
                                    ui.end_row();
                                }
                            });
                    });
            });
        });

        if self.data.clicked_item.is_some() {
            egui::TopBottomPanel::bottom("log_information")
                .resizable(true)
                .default_height(350.0)
                .max_height(450.0)
                .min_height(250.0)
                .show(ctx, |ui| {
                    self.data.preview_height = ui.available_height() + 2.0;
                    egui::Grid::new("log_information_grid")
                        .num_columns(2)
                        .min_col_width(ui.available_width() / 2.0)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                if ui.button("🗙 Close").clicked() {
                                    self.data.clicked_item = None;
                                }
                            });
                            ui.end_row();

                            ui.heading("Details");
                            ui.heading("Code");
                            ui.end_row();

                            ui.vertical(|ui| ui.label("Test"));
                            ui.vertical(|ui| ui.label("Test"));
                            ui.end_row();
                        });
                });
        } else {
            self.data.preview_height = 0.0;
        }
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        frame: &mut Frame<'_>,
        _storage: Option<&dyn Storage>,
    ) {
        let rx = Arc::clone(self.receiver.as_ref().unwrap());
        let received = Arc::clone(&self.data.received);
        let ctx = Arc::clone(&frame.repaint_signal());

        self.update_thread = Some(unsafe {
            ThreadBuilder::new()
                .name(format!("{}_update_thread", self.title))
                .spawn_unchecked(move || loop {
                    let recd = rx.lock().unwrap().recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front((recd, Local::now()));
                        ctx.request_repaint();
                    }
                })
                .expect("Could not start codeCTRL update thread")
        });
    }

    fn name(&self) -> &str { self.title.as_str() }
}
