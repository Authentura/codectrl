use chrono::{DateTime, Local};
use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex, RwLock},
};

pub type Received = Arc<RwLock<VecDeque<(String, DateTime<Local>)>>>;
pub type Receiver = Option<Arc<Mutex<mpsc::Receiver<String>>>>;
