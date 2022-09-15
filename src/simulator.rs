use crate::Key;

/// Stores the state required to simulate inputs.
///
/// On most platforms, this type will be a zero-sized type, but if the current platform requires
/// some kind of state when simulating keypresses, this type is responsible for managing that
/// state.
pub struct Simulator(crate::platform::Simulator);

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    #[inline]
    pub fn new() -> Result<Self, crate::Error> {
        crate::platform::Simulator::new()
            .map(Self)
            .map_err(crate::Error)
    }

    /// Sends a fake key press event to the top-level window.
    #[inline]
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        self.0.press_key(key).map_err(crate::Error)
    }

    /// Sends a fake key release event to the top-level window.
    #[inline]
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        self.0.release_key(key).map_err(crate::Error)
    }

    /// Sends a fake keystroke event to the top-level window.
    #[inline]
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        self.0.send_key(key).map_err(crate::Error)
    }
}
