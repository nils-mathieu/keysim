use std::fmt;
use std::fmt::Display;

/// An error that may occur when interacting with the X11 window manager.
#[derive(Debug)]
pub enum Error {
    /// An error occured when opening an X11 display instance.
    OpenError,
    /// The X11 server behaved in an unexpected way.
    UnexpectedError,
    /// The key is not supported.
    UnsupportedKey,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OpenError => f.write_str("failed to open a connection with the X server"),
            Self::UnexpectedError => f.write_str("the X server behaved in an unexpected way"),
            Self::UnsupportedKey => f.write_str("the X server does not support the requested key"),
        }
    }
}

impl std::error::Error for Error {}
