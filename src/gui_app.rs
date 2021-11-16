// In this file, we create the main graphical GUI for codeCTRL. A layout was
// proposed in issue #3, where Sebastian proposed a layout similar to this:
// _________________________________________________________________________
// | [ Filter search ] [x] Case insensitive [x] Regex | Some other settings|
// |-----------------------------------------------------------------------|
// |    _______________________________________________________________    |
// |    | x | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |    _______________________________________________________________    |
// |    |   | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |    _______________________________________________________________    |
// |    |   | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |_______________________________________________________________________|
// |  Log details                    |  Code snippet                       |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// ----------------------------------|--------------------------------------
//
// Further changes can be discussed and implemented at later dates, but this is
// the proposal so far.

use crate::{
    common::{Received, Receiver},
    components::{details_view, main_view},
};
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
pub enum Filter {
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
pub struct GuiAppState {
    pub search_filter: String,
    pub filter_by: Filter,
    #[serde(skip)]
    pub received: Received,
    pub is_case_sensitive: bool,
    pub is_using_regex: bool,
    pub is_newest_first: bool,
    #[serde(skip)]
    pub clicked_item: Option<(Log<String>, DateTime<Local>)>,
    #[serde(skip)]
    pub preview_height: f32,
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

                    // u1f5d9 = 🗙
                    if ui.button("\u{1f5d9}").clicked() {
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
                                "\u{2b07} Newest first" // u2b07 = ⬇
                            } else {
                                "\u{2b06} Newest last" // u2b06 = ⬆
                            },
                        )
                        .clicked()
                    {
                        self.data.is_newest_first = !self.data.is_newest_first;
                    }
                });
            });

        main_view(&mut self.data, ctx, &self.socket_address);

        if self.data.clicked_item.is_some() {
            details_view(&mut self.data, ctx);
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
