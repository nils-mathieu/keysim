use crate::{Button, Key};

/// The simulator on the Linux platform.
pub enum Simulator {
    #[cfg(feature = "x11")]
    X11(super::x11::Simulator),
}

impl Simulator {
    /// Creates a new [`Simulator`] for the current window manager.
    pub fn new() -> Result<Self, super::Error> {
        if let Some(kind) = std::env::var_os("XDG_SESSION_TYPE") {
            #[cfg(feature = "x11")]
            if kind == "x11" {
                return Self::new_x11();
            }
        }

        // If we were not able to determine which window manager to use, default to X11 if enabled.
        #[cfg(feature = "x11")]
        Self::new_x11()
    }

    /// Creates a new [`Simulator`] instance for the X11 window manager.
    #[cfg(feature = "x11")]
    pub fn new_x11() -> Result<Self, super::Error> {
        super::x11::Simulator::new()
            .map(Self::X11)
            .map_err(super::Error::X11)
    }

    /// Simulates a key press event.
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.press_key(key).map_err(super::Error::X11),
        }
    }

    /// Simulates a key release event.
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.release_key(key).map_err(super::Error::X11),
        }
    }

    /// Simulates a keystroke event.
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.send_key(key).map_err(super::Error::X11),
        }
    }

    /// Simulates a button press event.
    pub fn press_button(&self, button: Button) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.press_button(button).map_err(super::Error::X11),
        }
    }

    /// Simulates a key release event.
    pub fn release_button(&self, button: Button) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.release_button(button).map_err(super::Error::X11),
        }
    }

    /// Simulates a keystroke event.
    pub fn send_button(&self, button: Button) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.send_button(button).map_err(super::Error::X11),
        }
    }

    /// Simulates a character being typed.
    pub fn send_char(&self, c: char) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.send_char(c).map_err(super::Error::X11),
        }
    }

    /// Simulates characters being typed.
    pub fn send_chars(&self, it: impl Iterator<Item = char>) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.send_chars(it).map_err(super::Error::X11),
        }
    }

    /// Simulates characters being typed.
    pub fn send_str(&self, s: &str) -> Result<(), super::Error> {
        match self {
            #[cfg(feature = "x11")]
            Self::X11(sim) => sim.send_str(s).map_err(super::Error::X11),
        }
    }
}
