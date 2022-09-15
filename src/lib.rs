//! The `keysim` library can be used to simulate user inputs.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg_attr(target_os = "linux", path = "platform/linux/mod.rs")]
mod platform;

mod simulator;
pub use self::simulator::*;

mod key;
pub use self::key::*;

mod button;
pub use self::button::*;

mod error;
pub use self::error::*;