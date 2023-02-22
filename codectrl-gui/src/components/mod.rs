// region: modules

mod about_view;
mod about_view_components;
mod common;
mod details_view;
mod details_view_components;
mod main_view;
mod main_view_components;
mod message_preview_view;
mod settings_view;
mod settings_view_components;

// endregion

use common::regex_filter;

// region: re-exports

pub use about_view::*;
pub use details_view::*;
pub use main_view::*;
pub use message_preview_view::*;
pub use settings_view::*;

// endregion
