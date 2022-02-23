use chrono::{
    format::{format_item, StrftimeItems},
    DateTime, Local, NaiveDateTime,
};
use codectrl_logger::Log;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fmt,
    sync::{mpsc::Receiver as Rx, Arc, Mutex, RwLock},
};

pub type Received = Arc<RwLock<VecDeque<(Log<String>, DateTime<Local>)>>>;
pub type Receiver = Option<Arc<Mutex<Rx<Log<String>>>>>;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeFormatString(String);

impl TimeFormatString {
    pub fn new(fmt: &str) -> Self { Self(fmt.to_owned()) }
}

impl fmt::Display for TimeFormatString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strftime_items = StrftimeItems::new(&self.0);
        let datetime = NaiveDateTime::from_timestamp(45296, 0);

        for item in strftime_items {
            if let Err(_) = format_item(
                f,
                Some(&datetime.date()),
                Some(&datetime.time()),
                None,
                &item,
            ) {
                let _ = f.write_str("?");
            }
        }

        Ok(())
    }
}
