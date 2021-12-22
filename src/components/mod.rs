mod about_view;
mod common;
mod details_view;
mod details_view_components;
mod main_view;
mod message_preview_view;

include!(concat!(env!("OUT_DIR"), "/versions.include"));
include!(concat!(env!("OUT_DIR"), "/build_time.include"));

pub const GIT_COMMIT: &str = env!("GIT_COMMIT");
pub const GIT_BRANCH: &str = env!("GIT_BRANCH");

pub use about_view::*;
use common::regex_filter;
pub use details_view::*;
pub use main_view::*;
pub use message_preview_view::*;
