//! The platform implementation of the `keysim` crate on the Linux operating system.

#[cfg(feature = "x11")]
pub mod x11;

mod simulator;
pub use self::simulator::*;

mod error;
pub use self::error::*;
