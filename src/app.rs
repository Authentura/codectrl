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
    components::{
        about_view, dark_theme, details_view, fonts, main_view, main_view_empty,
        settings_view,
    },
    session::Session,
};
use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{CtxRef, Visuals};
use epi::{Frame, Storage};
use native_dialog::{FileDialog, MessageDialog};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fmt::{self, Display},
    fs::File,
    io::{BufReader, Write},
    process,
    sync::{mpsc::Receiver as Rx, Arc, Mutex, RwLock},
    thread::{Builder as ThreadBuilder, JoinHandle},
};

pub type Received = Arc<RwLock<VecDeque<(Log<String>, DateTime<Local>)>>>;
pub type Receiver = Option<Arc<Mutex<Rx<Log<String>>>>>;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Filter {
    Message,
    Time,
    FileName,
    Address,
    LineNumber,
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message => write!(f, "Message"),
            Self::Time => write!(f, "Time"),
            Self::FileName => write!(f, "File name"),
            Self::Address => write!(f, "Address"),
            Self::LineNumber => write!(f, "Line number"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum AboutState {
    About,
    Credits,
    License,
}

impl Default for AboutState {
    fn default() -> Self { Self::About }
}

impl Display for AboutState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::About => write!(f, "About"),
            Self::Credits => write!(f, "Credits"),
            Self::License => write!(f, "Licenses"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppState {
    pub search_filter: String,
    pub filter_by: Filter,
    #[serde(skip)]
    pub received: Received,
    pub do_scroll_to_selected_log: bool,
    #[serde(skip)]
    pub is_about_open: bool,
    #[serde(skip)]
    pub is_settings_open: bool,
    pub is_autosave: bool,
    pub is_case_sensitive: bool,
    pub is_copying_line_indicator: bool,
    pub is_copying_line_numbers: bool,
    pub is_message_preview_open: bool,
    pub is_newest_first: bool,
    pub is_using_regex: bool,
    #[serde(skip)]
    pub clicked_item: Option<(Log<String>, DateTime<Local>)>,
    #[serde(skip)]
    pub preview_height: f32,
    #[serde(skip)]
    pub about_state: AboutState,
    pub current_theme: Visuals,
    #[serde(skip)]
    pub copy_language: String,
    #[serde(skip)]
    pub alert_string: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            search_filter: "".into(),
            filter_by: Filter::Message,
            received: Arc::new(RwLock::new(VecDeque::new())),
            is_case_sensitive: false,
            is_using_regex: false,
            is_newest_first: true,
            is_about_open: false,
            is_message_preview_open: false,
            clicked_item: None,
            preview_height: 0.0,
            about_state: AboutState::About,
            current_theme: dark_theme(),
            copy_language: "".into(),
            is_copying_line_numbers: false,
            is_copying_line_indicator: false,
            do_scroll_to_selected_log: false,
            is_autosave: false,
            is_settings_open: false,
            alert_string: "".into(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct App {
    #[serde(skip)]
    pub receiver: Receiver,
    #[serde(skip)]
    update_thread: Option<JoinHandle<()>>,
    session: Session,
    data: AppState,
    title: String,
    socket_address: String,
}

impl App {
    pub fn new(title: &str, receiver: Rx<Log<String>>, socket_address: String) -> Self {
        Self {
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            data: AppState::default(),
            title: title.into(),
            session: Session::default(),
            socket_address,
        }
    }

    fn save_file(&mut self) {
        let file_path = if let Ok(file_path) = FileDialog::new()
            .set_filename(&format!(
                "{session_name}.cdctrl",
                session_name = self.session.session_name
            ))
            .add_filter("codeCTRL Session", &["cdctrl"])
            .show_save_single_file()
        {
            file_path
        } else {
            None
        };

        let file_path = match file_path {
            Some(file_path) => file_path,
            None => return,
        };

        self.session.app_state = self.data.clone();
        self.session.received = self.data.received.clone();

        let data = serde_cbor::to_vec(&self.session).expect("Could not serialise logs");

        let mut file = match File::create(&file_path) {
            Ok(file_path) => file_path,
            Err(error) => {
                let dialog = MessageDialog::new()
                    .set_title("Could not save file")
                    .set_text(&format!(
                        "Could not save file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy(),
                    ))
                    .show_alert();

                drop(dialog);

                return;
            },
        };

        if let Err(error) = file.write_all(data.as_slice()) {
            let dialog = MessageDialog::new()
                .set_title("Could not write to file")
                .set_text(&format!(
                    "Could not write to file \"{file_path}\": {error}",
                    file_path = file_path.to_string_lossy(),
                ))
                .show_alert();

            drop(dialog);
        }
    }

    fn load_file(&mut self) {
        let file_path = if let Ok(file_path) = FileDialog::new()
            .add_filter("codeCTRL Session", &["cdctrl"])
            .show_open_single_file()
        {
            file_path
        } else {
            None
        };

        let file_path = match file_path {
            Some(file_path) => file_path,
            None => return,
        };

        let file = match File::open(&file_path) {
            Ok(file_path) => file_path,
            Err(error) => {
                let dialog = MessageDialog::new()
                    .set_title("Could not open file")
                    .set_text(&format!(
                        "Could not open file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy(),
                    ))
                    .show_alert();

                drop(dialog);

                return;
            },
        };

        let reader = BufReader::new(file);

        let session: Session = match serde_cbor::from_reader(reader) {
            Ok(data) => {
                let dialog = MessageDialog::new()
                    .set_title("Successfully loaded file data")
                    .set_text("Loaded data from file successfully.")
                    .show_alert();

                drop(dialog);

                data
            },
            Err(error) => {
                let dialog = MessageDialog::new()
                    .set_title("Could not parse log data")
                    .set_text(&format!(
                        "Could not properly parse log data from file \"{file_path}\": \
                         {error}",
                        file_path = file_path.to_string_lossy(),
                    ))
                    .show_alert();

                drop(dialog);

                return;
            },
        };

        self.session = session.clone();
        self.data.received = session.received;
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        about_view(&mut self.data, ctx);
        settings_view(
            &mut self.session,
            &mut self.data.is_settings_open,
            &mut self.data.alert_string,
            ctx,
        );

        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Save").clicked() {
                            self.save_file();
                        }

                        if ui.button("Load").clicked() {
                            self.load_file();
                        }

                        ui.separator();

                        ui.checkbox(&mut self.data.is_autosave, "Auto Save");

                        if ui.button("Settings").clicked() {
                            self.data.is_settings_open = !self.data.is_settings_open;
                        }

                        ui.separator();

                        if ui.button("Quit").clicked() {
                            process::exit(0);
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.data.is_about_open = !self.data.is_about_open;
                        }
                    });

                    ui.separator();

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
                    ui.checkbox(
                        &mut self.data.do_scroll_to_selected_log,
                        "Scroll to selected log",
                    );

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

                    // u1f5d1 = ��
                    if ui.button("\u{1f5d1} Clear logs").clicked() {
                        if let Ok(mut received) = self.data.received.write() {
                            received.clear();
                            self.data.clicked_item = None;
                        }
                    }

                    ui.separator();

                    ui.label(format!("Listening on: {}", self.socket_address));
                });
            });

        let is_empty = {
            let received = Arc::clone(&self.data.received);

            let x = if let Ok(received) = received.read() {
                received.is_empty()
            } else {
                false
            };

            x
        };

        if is_empty {
            main_view_empty(ctx, &self.socket_address);
        } else {
            main_view(&mut self.data, &self.session, ctx);
        }

        if self.data.clicked_item.is_some() {
            details_view(&mut self.data, ctx);
        } else {
            self.data.preview_height = 0.0;
        }
    }

    fn setup(&mut self, ctx: &CtxRef, frame: &Frame, _storage: Option<&dyn Storage>) {
        let rx = Arc::clone(self.receiver.as_ref().unwrap());
        let received = Arc::clone(&self.data.received);
        ctx.set_visuals(self.data.current_theme.clone());
        ctx.set_fonts(fonts());

        self.update_thread = Some(unsafe {
            ThreadBuilder::new()
                .name(format!("{}_update_thread", self.title))
                .spawn_unchecked(move || loop {
                    let recd = rx.lock().unwrap().recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front((recd, Local::now()));
                        frame.request_repaint();
                    }
                })
                .expect("Could not start codeCTRL update thread")
        });
    }

    fn name(&self) -> &str { self.title.as_str() }
}
