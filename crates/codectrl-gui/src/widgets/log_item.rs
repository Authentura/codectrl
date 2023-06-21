use crate::widget::{ClickableWidget, Widget};
use codectrl_protobuf_bindings::data::Log;
use iced::Element;

pub struct LogItem<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    log: Log,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> LogItem<'a, Message, Renderer> where
    Renderer: iced_native::Renderer
{
}

impl<'a, Message, Renderer> Widget for LogItem<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    type Message = Message;

    fn draw(&self) -> iced::Element<'_, Self::Message> { todo!() }
}

impl<'a, Message, Renderer> ClickableWidget for LogItem<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn on_press(self, message: Self::Message) -> Self { todo!() }
}

impl<'a, Message, Renderer> Into<&'a dyn iced_native::Widget<Message, Renderer>>
    for LogItem<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn into(self) -> &'a dyn iced_native::Widget<Message, Renderer> {
        self.as_iced_widget()
    }
}
