/// A mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Button {
    /// The **Left** mouse button.
    Left,
    /// The **Middle** mouse button.
    Middle,
    /// The **Right** mouse button.
    Right,
    /// An extra mouse button.
    Extra(u8),
}
