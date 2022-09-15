//! This module provides the implementation of `keysim` when using the X11 window manager.

mod simulator;
pub use self::simulator::*;

mod error;
pub use self::error::*;

mod utils;
