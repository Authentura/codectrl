use iced::{Command, Element};
use iced_native::Theme;

pub trait Widget {
    type Message;

    fn draw(&self) -> Element<'_, Self::Message>;
    fn as_iced_widget<Renderer>(
        &self,
    ) -> &'_ dyn iced_native::Widget<Self::Message, Renderer>
    where
        Renderer: iced_graphics::Renderer<iced_wgpu::Backend, Theme>,
    {
        self.draw().as_widget()
    }
}

pub trait ClickableWidget: Widget {
    fn on_press(self, message: Self::Message) -> Self;
}
