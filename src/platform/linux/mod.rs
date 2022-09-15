//! The platform implementation of the `keysim` crate on the Linux operating system.

#[cfg(not(feature = "x11"))]
compile_error!("at the moment, only the X11 window manager is supported by `keysim`");

#[cfg(feature = "x11")]
pub mod x11;

mod simulator;
pub use self::simulator::*;

mod error;
pub use self::error::*;
