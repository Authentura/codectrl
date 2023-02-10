use crate::data::DEFAULT_FILENAME_FORMAT;

use authentura_egui_styling::FontSizes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationSettings {
    pub font_sizes: FontSizes,
    pub do_autosave: bool,
    pub filename_format: String,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            font_sizes: FontSizes::default(),
            do_autosave: false,
            filename_format: DEFAULT_FILENAME_FORMAT.into(),
        }
    }
}
