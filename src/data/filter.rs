use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Filter {
    Message,
    Time,
    FileName,
    Address,
    LineNumber,
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message => write!(f, "Message"),
            Self::Time => write!(f, "Time"),
            Self::FileName => write!(f, "File name"),
            Self::Address => write!(f, "Address"),
            Self::LineNumber => write!(f, "Line number"),
        }
    }
}
