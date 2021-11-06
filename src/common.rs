use chrono::{DateTime, Local};
use code_ctrl_logger::Log;
use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex, RwLock},
};

pub type Received = Arc<RwLock<VecDeque<(Log<String>, DateTime<Local>)>>>;
pub type Receiver = Option<Arc<Mutex<mpsc::Receiver<Log<String>>>>>;
