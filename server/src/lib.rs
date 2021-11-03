use std::{
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
    time::Duration,
};

#[derive(Copy, Clone)]
pub enum Mode {
    Full,
    Headless,
}

impl Default for Mode {
    fn default() -> Self { Self::Full }
}

#[derive(Clone)]
pub struct Server {
    mode: Mode,
    sender: SyncSender<String>,
}

impl Server {
    pub fn new(mode: Mode) -> (Self, Receiver<String>) {
        let (sender, receiver) = sync_channel(2);

        (Self { mode, sender }, receiver)
    }

    pub fn run_server(&self) {
        for i in 0..=100 {
            self.sender.send(format!("Test {}", i)).unwrap();
            thread::sleep(Duration::new(1, 0));
        }
    }
}
