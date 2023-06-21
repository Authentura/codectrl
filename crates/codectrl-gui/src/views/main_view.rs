use crate::{view::View, Message};

use iced::{widget::text, Command};
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
        let out = match *self {
            Self::NewestFirst => String::from("Newest first"),
            Self::OldestFirst => String::from("Oldest first"),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Main {
    pub scroll_to_selected_log: bool,
    pub log_appearance: LogAppearanceState,
}

impl View for Main {
    type Message = Message;

    fn title(&self) -> String { String::new() }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;

        match message {
            ScrollToSelectedLogChanged(state) => {
                self.scroll_to_selected_log = state;
                Command::none()
            },
            LogAppearanceStateChanged => {
                self.log_appearance.toggle();
                Command::none()
            },

            _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> { text("Main view").into() }
}
