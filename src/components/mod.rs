mod about_view;
mod common;
mod details_view;
mod main_view;

include!(concat!(env!("OUT_DIR"), "/versions.include"));
include!(concat!(env!("OUT_DIR"), "/build_time.include"));

pub const GIT_COMMIT: &str = env!("GIT_COMMIT");
pub const GIT_BRANCH: &str = env!("GIT_BRANCH");

pub use about_view::*;
use common::regex_filter;
pub use details_view::*;
pub use main_view::*;
