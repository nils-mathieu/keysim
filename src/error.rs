use std::fmt;
use std::fmt::Debug;

/// An error that might occur when interacting when simulating user inputs.
pub struct Error(pub(crate) crate::platform::Error);

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
