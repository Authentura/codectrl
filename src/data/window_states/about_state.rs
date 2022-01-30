use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum AboutState {
    About,
    Credits,
    License,
}

impl Default for AboutState {
    fn default() -> Self { Self::About }
}

impl Display for AboutState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::About => write!(f, "About"),
            Self::Credits => write!(f, "Credits"),
            Self::License => write!(f, "Licenses"),
        }
    }
}
