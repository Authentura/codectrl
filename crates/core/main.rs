use codectrl_gui::{
    iced::{self, Application, Settings},
    App, Flags,
};

fn main() -> iced::Result {
    App::run(Settings {
        id: Some(String::from("CodeCTRL")),
        flags: Flags::default(),
        text_multithreading: true,
        antialiasing: true,
        ..Settings::default()
    })
}
