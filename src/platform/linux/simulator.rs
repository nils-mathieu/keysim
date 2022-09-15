/// The simulator on the Linux platform.
pub enum Simulator {
    #[cfg(feature = "x11")]
    X11(super::x11::Simulator),
}

impl Simulator {
    /// Creates a new [`Simulator`] for the current window manager.
    pub fn new() -> Result<Self, super::Error> {
        if let Some(kind) = std::env::var_os("XDG_SESSION_TYPE") {
            if kind == "x11" {
                return Self::new_x11();
            }
        }

        // If we were not able to determine which window manager to use, default to X11.
        Self::new_x11()
    }

    /// Creates a new [`Simulator`] instance for the X11 window manager.
    #[cfg(feature = "x11")]
    pub fn new_x11() -> Result<Self, super::Error> {
        super::x11::Simulator::new()
            .map(Self::X11)
            .map_err(super::Error::X11)
    }
}