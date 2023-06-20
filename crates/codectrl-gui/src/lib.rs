pub use iced;

use iced::{
    widget::{column, text},
    Element, Sandbox,
};

pub struct App;

impl Sandbox for App {
    type Message = ();

    fn new() -> Self { App }

    fn title(&self) -> String { String::from("CodeCTRL") }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> { column![text("Hello")].into() }
}
