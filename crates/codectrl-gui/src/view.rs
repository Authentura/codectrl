use iced::{Command, Element};

pub trait View {
    type Message;

    fn title(&self) -> String;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn view(&self) -> Element<'_, Self::Message>;

    fn send_message(&self, message: Self::Message) -> Command<Self::Message>
    where
        Self::Message: 'static + Send,
    {
        Command::perform((|| async {})(), |_| message)
    }
}
