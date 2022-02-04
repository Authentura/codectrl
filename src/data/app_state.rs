use super::{window_states::AboutState, ApplicationSettings, Filter, Received};
use crate::components::dark_theme;

use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::Visuals;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::{Arc, RwLock},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppState {
    pub search_filter: String,
    pub filter_by: Filter,
    #[serde(skip)]
    pub received: Received,
    pub do_scroll_to_selected_log: bool,
    #[serde(skip)]
    pub is_about_open: bool,
    #[serde(skip)]
    pub is_settings_open: bool,
    pub is_autosave: bool,
    pub is_case_sensitive: bool,
    pub is_copying_line_indicator: bool,
    pub is_copying_line_numbers: bool,
    pub is_message_preview_open: bool,
    pub is_newest_first: bool,
    pub is_using_regex: bool,
    #[serde(skip)]
    pub clicked_item: Option<(Log<String>, DateTime<Local>)>,
    #[serde(skip)]
    pub preview_height: f32,
    #[serde(skip)]
    pub about_state: AboutState,
    pub current_theme: Visuals,
    #[serde(skip)]
    pub copy_language: String,
    #[serde(skip)]
    pub alert_string: String,
    pub message_alerts: BTreeSet<String>,
    #[serde(skip)]
    pub session_timestamp: String,
    pub application_settings: ApplicationSettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            search_filter: "".into(),
            filter_by: Filter::Message,
            received: Arc::new(RwLock::new(VecDeque::new())),
            is_case_sensitive: false,
            is_using_regex: false,
            is_newest_first: true,
            is_about_open: false,
            is_message_preview_open: false,
            clicked_item: None,
            preview_height: 0.0,
            about_state: AboutState::About,
            current_theme: dark_theme(),
            copy_language: "".into(),
            is_copying_line_numbers: false,
            is_copying_line_indicator: false,
            do_scroll_to_selected_log: false,
            is_autosave: false,
            is_settings_open: false,
            alert_string: "".into(),
            message_alerts: BTreeSet::new(),
            session_timestamp: "".into(),
            application_settings: ApplicationSettings::default(),
        }
    }
}
