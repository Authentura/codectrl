// region: modules

mod app_state;
mod filter;
mod settings;
mod types;

pub mod window_states;

// endregion
// region: re-exports

pub use app_state::AppState;
pub use filter::Filter;
pub use settings::ApplicationSettings;
pub use types::{Received, TimeFormatString};

// endregion

pub const DEFAULT_FILENAME_FORMAT: &str = "session_%F %H_%M_%S";
pub const ISO_8601_TIME_FORMAT: &str = "%F %X (%Z)";
pub const LOCALE_TIME_FORMAT: &str = "%c (%Z)";
