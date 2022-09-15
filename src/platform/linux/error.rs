use core::fmt;
use core::fmt::Debug;

/// An error that may occur when simulating user inputs on the Linux operating system.
pub enum Error {
    #[cfg(feature = "x11")]
    X11(super::x11::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X11(e) => Debug::fmt(e, f),
        }
    }
}
