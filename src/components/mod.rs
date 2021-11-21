mod about_view;
mod details_view;
mod main_view;

include!(concat!(env!("OUT_DIR"), "/versions.include"));
include!(concat!(env!("OUT_DIR"), "/build_time.include"));

pub use about_view::*;
pub use details_view::*;
pub use main_view::*;
