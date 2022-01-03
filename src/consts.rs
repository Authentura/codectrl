include!(concat!(env!("OUT_DIR"), "/versions.include"));
include!(concat!(env!("OUT_DIR"), "/build_time.include"));

pub const GIT_COMMIT: &str = env!("GIT_COMMIT");
pub const GIT_BRANCH: &str = env!("GIT_BRANCH");

pub const OTF_FONT_REGULAR: &[u8] =
    include_bytes!("../assets/fonts/red-hat/RedHatDisplay-Medium.otf");
pub const OTF_FONT_MONOSPACE: &[u8] =
    include_bytes!("../assets/fonts/red-hat/RedHatMono-Regular.otf");
