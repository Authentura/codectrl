use dark_light::{self, Mode as ThemeMode};
pub use iced;
use iced::{
    executor,
    widget::{button, checkbox, column, container, row, text, text_input, Rule, Space},
    window::close,
    Alignment, Application, Command, Element, Length, Sandbox, Theme,
};
use iced_aw::menu::{ItemWidth, MenuBar, MenuTree, PathHighlight};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum LogAppearanceState {
    #[default]
    NewestFirst,
    OldestFirst,
}

impl LogAppearanceState {
    fn toggle(&mut self) {
        if *self == Self::NewestFirst {
            *self = Self::OldestFirst;
        } else {
            *self = Self::NewestFirst;
        }
    }
}

impl fmt::Display for LogAppearanceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out;

        match *self {
            Self::NewestFirst => out = String::from("Newest first"),
            Self::OldestFirst => out = String::from("Oldest first"),
        }

        write!(f, "{out}")
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    FilterTextChanged(String),
    ClearFilterText,
    FilterCaseSenitivityChanged(bool),
    FilterRegexChanged(bool),
    ScrollToSelectedLogChanged(bool),
    LogAppearanceStateChanged,
    Quit,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum ViewState {
    Searching {
        filter: String,
        case_sensitive: bool,
        regex_sensitive: bool,
    },
    #[default]
    Main,
}

#[derive(Debug, Clone, Default)]
pub struct App {
    case_sensitive: bool,
    filter: String,
    regex_sensitive: bool,
    scroll_to_selected_log: bool,
    log_appearance: LogAppearanceState,
    view_state: ViewState,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String { String::from("CodeCTRL") }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        use Message::*;

        match message {
            FilterTextChanged(text) => {
                self.filter = text;

                Command::none()
            },
            ClearFilterText => {
                self.filter.clear();

                Command::none()
            },
            FilterCaseSenitivityChanged(state) => {
                self.case_sensitive = state;

                Command::none()
            },
            FilterRegexChanged(state) => {
                self.regex_sensitive = state;

                Command::none()
            },
            ScrollToSelectedLogChanged(state) => {
                self.scroll_to_selected_log = state;

                Command::none()
            },
            LogAppearanceStateChanged => {
                self.log_appearance.toggle();

                Command::none()
            },
            Quit => close(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let file_menu_tree = MenuTree::with_children(
            button("File"),
            vec![
                MenuTree::new(button("Save project").width(Length::Fill)),
                MenuTree::new(button("Open project").width(Length::Fill)),
                MenuTree::new(button("Settings").width(Length::Fill)),
                MenuTree::new(button("Log out").width(Length::Fill)),
                MenuTree::new(button("Quit").width(Length::Fill)),
            ],
        );

        let help_menu_tree = MenuTree::with_children(
            button("Help"),
            vec![MenuTree::new(button("About").width(Length::Fill))],
        );

        let menu_bar = MenuBar::new(vec![file_menu_tree, help_menu_tree])
            .path_highlight(Some(PathHighlight::Full))
            .spacing(1.0)
            .padding(2.0);

        let side_bar = container(
            column![
                text_input("Filter", &self.filter).on_input(Message::FilterTextChanged),
                button("Clear").on_press(Message::ClearFilterText),
                checkbox(
                    "Case sensitive",
                    self.case_sensitive,
                    Message::FilterCaseSenitivityChanged
                ),
                checkbox("Regex", self.regex_sensitive, Message::FilterRegexChanged),
                Rule::horizontal(1.0),
                checkbox(
                    "Scroll to selected log",
                    self.scroll_to_selected_log,
                    Message::ScrollToSelectedLogChanged
                ),
                row![
                    text("Sort logs by: "),
                    button(text(&self.log_appearance.to_string()))
                        .on_press(Message::LogAppearanceStateChanged)
                ]
                .align_items(Alignment::Center)
            ]
            .align_items(Alignment::Start)
            .spacing(4.0)
            .padding(10.0),
        );

        match self.view_state {
            _ => column![
                menu_bar,
                Rule::horizontal(1.0),
                row![
                    side_bar.width(Length::FillPortion(2)),
                    Rule::vertical(1.0),
                    column![text(&self.filter)]
                        .width(Length::FillPortion(6))
                        .padding(10.0),
                ],
            ]
            .into(),
        }
    }

    fn theme(&self) -> Theme {
        let mode = dark_light::detect();

        match mode {
            ThemeMode::Dark | ThemeMode::Default => Theme::Dark,
            ThemeMode::Light => Theme::Light,
        }
    }
}
