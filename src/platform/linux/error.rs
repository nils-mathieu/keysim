use std::fmt;
use std::fmt::{Debug, Display};

/// An error that may occur when simulating user inputs on the Linux operating system.
pub enum Error {
    #[cfg(feature = "x11")]
    X11(super::x11::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(e) => Debug::fmt(e, f),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(e) => Some(e),
        }
    }
}
