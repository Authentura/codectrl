use crate::{view::View, Message, ViewState};
use iced::{widget::text, Command};

#[derive(Debug, Clone, Default)]
pub struct Searching {
    pub filter: String,
    pub case_sensitive: bool,
    pub regex_sensitive: bool,
}

impl View for Searching {
    type Message = Message;

    fn title(&self) -> String { String::from("Searching logs...") }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;

        match message.clone() {
            FilterTextChanged(text) => {
                self.filter = text;

                if self.filter.is_empty() {
                    return self.send_message(Message::UpdateViewState(ViewState::Main));
                }

                self.send_message(Message::UpdateViewState(ViewState::Searching))
            },
            ClearFilterText =>
                self.send_message(Message::FilterTextChanged(String::new())),
            FilterCaseSenitivityChanged(state) => {
                self.case_sensitive = state;

                Command::none()
            },
            FilterRegexChanged(state) => {
                self.regex_sensitive = state;

                Command::none()
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text(format!("Searching view: {}", self.filter)).into()
    }
}
