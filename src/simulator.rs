/// Stores the state required to simulate inputs.
///
/// On most platforms, this type will be a zero-sized type, but if the current platform requires
/// some kind of state when simulating keypresses, this type is responsible for managing that
/// state.
pub struct Simulator(crate::platform::Simulator);

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    pub fn new() -> Result<Self, crate::Error> {
        crate::platform::Simulator::new()
            .map(Self)
            .map_err(crate::Error)
    }
}
