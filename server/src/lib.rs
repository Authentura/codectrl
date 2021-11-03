#[derive(Copy, Clone)]
pub enum Mode {
    Full,
    Headless,
}

impl Default for Mode {
    fn default() -> Self { Self::Full }
}

pub async fn server(mode: Mode) {
    match mode {
        Mode::Full => println!("Running in graphical mode"),
        Mode::Headless => println!("Running in text-only mode"),
    }
}
