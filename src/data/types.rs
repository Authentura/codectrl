use chrono::{DateTime, Local};
use codectrl_logger::Log;
use std::{
    collections::VecDeque,
    sync::{mpsc::Receiver as Rx, Arc, Mutex, RwLock},
};

pub type Received = Arc<RwLock<VecDeque<(Log<String>, DateTime<Local>)>>>;
pub type Receiver = Option<Arc<Mutex<Rx<Log<String>>>>>;
