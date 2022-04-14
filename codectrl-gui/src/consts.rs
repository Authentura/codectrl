include!(concat!(env!("OUT_DIR"), "/versions.include"));
include!(concat!(env!("OUT_DIR"), "/build_time.include"));

pub const GIT_COMMIT: &str = env!("GIT_COMMIT");
pub const GIT_BRANCH: &str = env!("GIT_BRANCH");
