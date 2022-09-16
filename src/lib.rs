//! The `keysim` library can be used to simulate user inputs.

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!("the current compilation target is not supported by the `keysim` crate");

#[cfg_attr(target_os = "linux", path = "platform/linux/mod.rs")]
#[cfg_attr(target_os = "windows", path = "platform/windows/mod.rs")]
mod platform;

mod simulator;
pub use self::simulator::*;

mod key;
pub use self::key::*;

mod button;
pub use self::button::*;

mod error;
pub use self::error::*;
