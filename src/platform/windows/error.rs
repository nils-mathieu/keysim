use std::fmt;
use std::fmt::Display;

use crate::Key;

/// An error that might occur when interacting with the Windows operating system.
#[derive(Debug)]
pub enum Error {
    Blocked,
    UnsupportedKey(Key),
    UnsupportedChar(char),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Blocked => f.write_str("the inputs were blocked by another thread"),
            Self::UnsupportedKey(k) => write!(f, "the key '{k:?}' is not supported"),
            Self::UnsupportedChar(c) => write!(f, "the character {c:?} is not supported"),
        }
    }
}

impl std::error::Error for Error {}
