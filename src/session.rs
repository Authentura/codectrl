use crate::app::{AppState, Received};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::RwLock,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub session_name: String,
    pub app_state: AppState,
    pub received: Received,
    pub message_alerts: BTreeSet<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_name: format!(
                "{date}-session",
                date = Local::now().format("%Y-%m-%d")
            ),
            received: Received::new(RwLock::new(VecDeque::new())),
            app_state: AppState::default(),
            message_alerts: BTreeSet::new(),
        }
    }
}
