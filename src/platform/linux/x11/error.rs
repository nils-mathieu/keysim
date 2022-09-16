use std::fmt;
use std::fmt::Display;

use crate::Key;

/// An error that may occur when interacting with the X11 window manager.
#[derive(Debug)]
pub enum Error {
    /// An error occured when opening an X11 display instance.
    OpenDisplay,
    /// The X11 server behaved in an unexpected way.
    Unexpected,
    /// The key is not supported.
    UnsupportedKey(Key),
    /// The character is not supported.
    UnsupportedChar(char),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OpenDisplay => f.write_str("failed to open a connection with the X server"),
            Self::Unexpected => f.write_str("the X server behaved in an unexpected way"),
            Self::UnsupportedKey(k) => write!(f, "the X server does not support the '{k:?}' key"),
            Self::UnsupportedChar(c) => write!(f, "the X server does not support the {c:?} char"),
        }
    }
}

impl std::error::Error for Error {}
