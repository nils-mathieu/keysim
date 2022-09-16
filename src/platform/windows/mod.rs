//! The platform implementation of the `keysim` crate on the Windows operating system.

mod error;
pub use self::error::*;

mod simulator;
pub use self::simulator::*;

mod sys;
mod utils;
