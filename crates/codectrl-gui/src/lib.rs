#![feature(associated_type_defaults)]
#![warn(clippy::perf, clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

mod view;
mod views;

use crate::view::View;

use anyhow::Error;
use codectrl_server::{self, ServerResult};
use dark_light::{self, Mode as ThemeMode};
pub use iced;
use iced::{
    executor, subscription,
    widget::{button, checkbox, column, container, row, text, text_input, Rule},
    window::close,
    Alignment, Application, Command, Element, Length, Subscription, Theme,
};
use iced_aw::{
    helpers::{menu_bar, menu_tree},
    menu::PathHighlight,
    menu_tree, quad,
    split::Axis,
    Split,
};
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::{self, error::TryRecvError};

#[derive(Debug, Clone)]
pub enum Message {
    // main view
    ScrollToSelectedLogChanged(bool),
    LogAppearanceStateChanged,

    // searching view
    FilterTextChanged(String),
    ClearFilterText,
    FilterCaseSenitivityChanged(bool),
    FilterRegexChanged(bool),

    // general
    UpdateViewState(ViewState),
    SplitResize(u16),
    ServerStarted {
        server_result: Arc<ServerResult>,
        details: (String, u32),
    },
    ServerConnection(String, u32),
    ShowServerError(Arc<anyhow::Error>),
    SetServerErrorChannel(Arc<mpsc::UnboundedReceiver<anyhow::Error>>),
    ServerNoOp,
    Quit,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum ViewState {
    Searching,
    #[default]
    Main,
}

fn separator<'a, Message>()
-> iced_aw::menu::menu_tree::MenuTree<'a, Message, iced::Renderer> {
    menu_tree!(quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub struct Flags {
    host: String,
    port: u32,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 3002,
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    // server communication
    server_errors: Option<Arc<Mutex<mpsc::UnboundedReceiver<anyhow::Error>>>>,
    host: String,
    port: u32,

    // splits
    split_size: Option<u16>,

    // views and view state
    view_state: ViewState,
    main_view: views::Main,
    searching_view: views::Searching,
}

impl Default for App {
    fn default() -> Self { Self::new_no_server(Flags::default()) }
}

impl App {
    fn new_no_server(flags: Flags) -> Self {
        Self {
            host: flags.host,
            port: flags.port,
            server_errors: None,
            split_size: Some(208),
            view_state: ViewState::default(),
            main_view: views::Main::default(),
            searching_view: views::Searching::default(),
        }
    }

    fn send_message(&self, message: Message) -> Command<Message> {
        Command::perform(async {}, |_| message)
    }
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            App::default(),
            Command::perform(
                codectrl_server::run_server(
                    Some(flags.host.clone()),
                    Some(flags.port),
                    None,
                    None,
                ),
                move |result| Message::ServerStarted {
                    server_result: Arc::new(result),
                    details: (flags.host, flags.port),
                },
            ),
        )
    }

    fn title(&self) -> String { String::from("CodeCTRL") }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        use Message::*;

        match message {
            ScrollToSelectedLogChanged(_) | LogAppearanceStateChanged =>
                self.main_view.update(message),

            FilterTextChanged(_)
            | ClearFilterText
            | FilterCaseSenitivityChanged(_)
            | FilterRegexChanged(_) => self.searching_view.update(message),

            UpdateViewState(state) => {
                self.view_state = state;
                Command::none()
            },
            SplitResize(size) => {
                self.split_size = Some(size);
                Command::none()
            },
            ServerStarted {
                server_result,
                details: (host, port),
            } => match Arc::try_unwrap(server_result) {
                Ok(x) if matches!(x, Ok(_)) => {
                    let channel = x.unwrap();
                    self.host = host;
                    self.port = port;

                    self.send_message(SetServerErrorChannel(Arc::new(channel)))
                },
                Ok(x) if matches!(x, Err(_)) => {
                    let error = x.unwrap_err();

                    self.send_message(ShowServerError(Arc::new(error)))
                },
                Ok(_) => unreachable!(),
                Err(_) => self.send_message(ShowServerError(Arc::new(Error::msg(
                    "Could not unwrap server result",
                )))),
            },
            SetServerErrorChannel(rx) => {
                if let Ok(rx) = Arc::try_unwrap(rx) {
                    self.server_errors = Some(Arc::new(Mutex::new(rx)));
                } else {
                    return self.send_message(ShowServerError(Arc::new(
                        anyhow::Error::msg("Could not unwrap server error receiver"),
                    )));
                }

                Command::none()
            },
            ServerConnection(host, port) => {
                dbg!(&host);
                dbg!(&port);

                Command::none()
            },
            ShowServerError(error) => {
                dbg!(error);
                Command::none()
            },
            ServerNoOp => Command::none(),
            Quit => close(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
        // subscription::run(|| ).into())
    }

    fn view(&self) -> Element<Self::Message> {
        let file_menu_tree = menu_tree(
            button("File"),
            vec![
                menu_tree!(button("Save project").width(Length::Fill)),
                menu_tree!(button("Open project").width(Length::Fill)),
                separator(),
                menu_tree!(button("Settings").width(Length::Fill)),
                menu_tree!(button("Log out").width(Length::Fill)),
                separator(),
                menu_tree!(button("Quit").width(Length::Fill).on_press(Message::Quit)),
            ],
        );

        let help_menu_tree = menu_tree(
            button("Help"),
            vec![menu_tree!(button("About").width(Length::Fill))],
        );

        let menu_bar = menu_bar(vec![file_menu_tree, help_menu_tree])
            .path_highlight(Some(PathHighlight::Full))
            .spacing(1.0)
            .padding(2.0);

        let side_bar = container(
            column![
                text_input("Filter", &self.searching_view.filter)
                    .on_input(Message::FilterTextChanged),
                button("Clear").on_press(Message::ClearFilterText),
                checkbox(
                    "Case sensitive",
                    self.searching_view.case_sensitive,
                    Message::FilterCaseSenitivityChanged
                ),
                checkbox(
                    "Regex",
                    self.searching_view.regex_sensitive,
                    Message::FilterRegexChanged
                ),
                Rule::horizontal(1.0),
                checkbox(
                    "Scroll to selected log",
                    self.main_view.scroll_to_selected_log,
                    Message::ScrollToSelectedLogChanged
                ),
                row![
                    text("Sort logs by: "),
                    button(text(&self.main_view.log_appearance))
                        .on_press(Message::LogAppearanceStateChanged)
                ]
                .align_items(Alignment::Center)
            ]
            .align_items(Alignment::Start)
            .spacing(4.0)
            .padding(10.0),
        );

        column![
            menu_bar,
            Rule::horizontal(1.0),
            row![
                Split::new(
                    side_bar.width(Length::Fill),
                    container(match self.view_state {
                        ViewState::Main => self.main_view.view(),
                        ViewState::Searching => self.searching_view.view(),
                    })
                    .width(Length::Fill)
                    .padding(10.0),
                    self.split_size,
                    Axis::Vertical,
                    Message::SplitResize
                )
                .min_size_first(208)
                .min_size_second(600)
            ]
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        let mode = dark_light::detect();

        match mode {
            ThemeMode::Dark | ThemeMode::Default => Theme::Dark,
            ThemeMode::Light => Theme::Light,
        }
    }
}
