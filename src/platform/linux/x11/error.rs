/// An error that may occur when interacting with the X11 window manager.
#[derive(Debug)]
pub enum Error {
    /// An error occured when opening an X11 display instance.
    OpenError,
    /// The X11 server behaved in an unexpected way.
    UnexpectedError,
    /// The key is not supported.
    UnsupportedKey,
}
