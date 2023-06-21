use codectrl_gui::{
    iced::{self, Application, Settings},
    App,
};

fn main() -> iced::Result {
    App::run(Settings {
        id: Some(String::from("CodeCTRL")),
        flags: (),
        text_multithreading: true,
        antialiasing: true,
        ..Settings::default()
    })
}
