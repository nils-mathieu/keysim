use bitflags::bitflags;

/// Keyboard keys.
///
/// This enumeration can be used when simulating a specific virtual-key code (or symbol), rather
/// than a specific physical key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// The **A** key.
    A,
    /// The **B** key.
    B,
    /// The **C** key.
    C,
    /// The **D** key.
    D,
    /// The **E** key.
    E,
    /// The **F** key.
    F,
    /// The **G** key.
    G,
    /// The **H** key.
    H,
    /// The **I** key.
    I,
    /// The **J** key.
    J,
    /// The **K** key.
    K,
    /// The **L** key.
    L,
    /// The **M** key.
    M,
    /// The **N** key.
    N,
    /// The **O** key.
    O,
    /// The **P** key.
    P,
    /// The **Q** key.
    Q,
    /// The **R** key.
    R,
    /// The **S** key.
    S,
    /// The **T** key.
    T,
    /// The **U** key.
    U,
    /// The **V** key.
    V,
    /// The **W** key.
    W,
    /// The **X** key.
    X,
    /// The **Y** key.
    Y,
    /// The **Z** key.
    Z,

    /// The **F1** key.
    F1,
    /// The **F2** key.
    F2,
    /// The **F3** key.
    F3,
    /// The **F4** key.
    F4,
    /// The **F5** key.
    F5,
    /// The **F6** key.
    F6,
    /// The **F7** key.
    F7,
    /// The **F8** key.
    F8,
    /// The **F9** key.
    F9,
    /// The **F10** key.
    F10,
    /// The **F11** key.
    F11,
    /// The **F12** key.
    F12,
    /// The **F13** key.
    F13,
    /// The **F14** key.
    F14,
    /// The **F15** key.
    F15,
    /// The **F16** key.
    F16,
    /// The **F17** key.
    F17,
    /// The **F18** key.
    F18,
    /// The **F19** key.
    F19,
    /// The **F20** key.
    F20,
    /// The **F21** key.
    F21,
    /// The **F22** key.
    F22,
    /// The **F23** key.
    F23,
    /// The **F24** key.
    F24,

    /// The **0** key, above the letters.
    #[doc(alias = "0")]
    Zero,
    /// The **1** key, above the letters.
    #[doc(alias = "1")]
    One,
    /// The **2** key, above the letters.
    #[doc(alias = "2")]
    Two,
    /// The **3** key, above the letters.
    #[doc(alias = "3")]
    Three,
    /// The **4** key, above the letters.
    #[doc(alias = "4")]
    Four,
    /// The **5** key, above the letters.
    #[doc(alias = "5")]
    Five,
    /// The **6** key, above the letters.
    #[doc(alias = "6")]
    Six,
    /// The **7** key, above the letters.
    #[doc(alias = "7")]
    Seven,
    /// The **8** key, above the letters.
    #[doc(alias = "8")]
    Eight,
    /// The **9** key, above the letters.
    #[doc(alias = "9")]
    Nine,

    /// The **Escape** key.
    Escape,
    /// The **Tab** key.
    Tab,
    /// The **Caps Lock** key.
    ///
    /// This key is sometimes named **Capital**.
    #[doc(alias = "Capital")]
    CapsLock,
    /// The left **Shift** key.
    LeftShift,
    /// The left **Control** key.
    LeftControl,
    /// The left **Alt** key.
    LeftAlt,
    /// The left **Meta** key.
    ///
    /// This key is sometimes named **Windows**, **Super**, or **Command**.
    #[doc(alias = "Windows")]
    #[doc(alias = "Super")]
    #[doc(alias = "Command")]
    LeftMeta,
    /// The **Space** bar.
    Space,
    /// The right **Meta** key.
    ///
    /// This key is sometimes named **Windows**, **Super**, or **Command**.
    #[doc(alias = "Windows")]
    #[doc(alias = "Super")]
    #[doc(alias = "Command")]
    RightMeta,
    /// The right **Alt** key.
    RightAlt,
    /// The right **Control** key.
    RightControl,
    /// The right **Shift** key.
    RightShift,
    /// The **Enter** key.
    ///
    /// This key is sometimes named **Return**.
    #[doc(alias = "Return")]
    Enter,
    /// The **Backspace** key.
    Backspace,

    /// The **Insert** key.
    Insert,
    /// The **Delete** key.
    Delete,
    /// The **Home** key.
    Home,
    /// The **End** key.
    End,
    /// The **Page Up** key.
    ///
    /// This key is sometimes named **Previous**.
    #[doc(alias = "Previous")]
    PageUp,
    /// The **Page Down** key.
    ///
    /// This key is sometimes named **Next**.
    #[doc(alias = "Next")]
    PageDown,

    /// The **Up** arrow key.
    ArrowUp,
    /// The **Down** arrow key.
    ArrowDown,
    /// The **Left** arrow key.
    ArrowLeft,
    /// The **Right** arrow key.
    ArrowRight,

    /// The **Num Lock** key.
    ///
    /// This key is sometimes named **Clear**.
    #[doc(alias = "Clear")]
    NumLock,
    /// The **=** key, on the numpad.
    #[doc(alias = "=")]
    NumpadEqual,
    /// The **/** key, on the numpad.
    #[doc(alias = "/")]
    NumpadDivide,
    /// The **\*** key, on the numpad.
    #[doc(alias = "*")]
    NumpadMultiply,
    /// The **+** key, on the numpad.
    #[doc(alias = "+")]
    NumpadAdd,
    /// The **Enter** key, on the numpad.
    NumpadEnter,
    /// The **.** key, on the numpad.
    #[doc(alias = ".")]
    NumpadDecimal,
    /// The **0** key, on the numpad.
    Numpad0,
    /// The **1** key, on the numpad.
    Numpad1,
    /// The **2** key, on the numpad.
    Numpad2,
    /// The **3** key, on the numpad.
    Numpad3,
    /// The **4** key, on the numpad.
    Numpad4,
    /// The **5** key, on the numpad.
    Numpad5,
    /// The **6** key, on the numpad.
    Numpad6,
    /// The **7** key, on the numpad.
    Numpad7,
    /// The **8** key, on the numpad.
    Numpad8,
    /// The **9** key, on the numpad.
    Numpad9,

    /// The **Volume Up** key.
    VolumeUp,
    /// The **Volume Down** key.
    VolumeDown,
    /// The **Volume Mute** key.
    VolumeMute,
    /// The **Media Next** key.
    MediaNext,
    /// The **Media Previous** key.
    MediaPrevious,
    /// The **Media Stop** key.
    MediaStop,
    /// The **Media Pause** key.
    MediaPause,
}

bitflags! {
    /// A collection of modifiers that may be added to a key.
    pub struct Modifiers: u8 {
        /// Whether the **SHIFT** modifier should be enabled for the keystroke.
        const SHIFT = 1 << 0;
        /// Whether the **CONTROL** modifier should be enabled for the keystroke.
        const CONTROL = 1 << 0;
    }
}
