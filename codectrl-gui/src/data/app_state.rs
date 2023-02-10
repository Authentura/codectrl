use super::{window_states::AboutState, ApplicationSettings, Filter, Received};
use crate::data::DEFAULT_FILENAME_FORMAT;

use authentura_egui_styling::dark_theme;
use chrono::{DateTime, Local};
use codectrl_protobuf_bindings::{
    data::Log,
    logs_service::{Connection, ServerDetails},
};
use egui::Visuals;
#[cfg(target_arch = "wasm32")]
use instant::Instant;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
use std::{
    collections::{BTreeSet, VecDeque},
    sync::{Arc, RwLock},
};

pub fn time_details_last_checked_default() -> Instant { Instant::now() }
pub fn refresh_server_details_default() -> bool { true }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppState {
    #[serde(skip)]
    pub server_details: Option<ServerDetails>,
    #[serde(skip)]
    pub grpc_client_connection: Option<Connection>,
    #[serde(skip, default = "time_details_last_checked_default")]
    pub time_details_last_checked: Instant,
    #[serde(skip, default = "refresh_server_details_default")]
    pub refresh_server_details: bool,
    pub search_filter: String,
    pub filter_by: Filter,
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
    pub clicked_item: Option<(Log, DateTime<Local>)>,
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
    pub filename_format: String,
    pub preserve_session: bool,
    #[serde(skip)]
    pub code_hash: u128,
    #[serde(skip)]
    pub code_job: egui::text::LayoutJob,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            server_details: None,
            grpc_client_connection: None,
            time_details_last_checked: time_details_last_checked_default(),
            refresh_server_details: refresh_server_details_default(),
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
            filename_format: DEFAULT_FILENAME_FORMAT.into(),
            preserve_session: true,
            code_hash: 0,
            code_job: egui::text::LayoutJob::default(),
        }
    }
}
