/// The simulator used when using the X11 window manager.
pub struct Simulator {
    /// The display used that's being used to simulate keystrokes.
    display: *mut x11::xlib::Display,
}

// TODO:
//  Figure out wether the `Simulator` type should be thread safe or not.

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    pub fn new() -> Result<Self, super::Error> {
        // Safety:
        //  Calling this function with a null pointer as input is always valid.
        let display = unsafe { x11::xlib::XOpenDisplay(core::ptr::null()) };

        if display.is_null() {
            return Err(super::Error::OpenError);
        }

        Ok(Self { display })
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        // Safety:
        //  By invariant, we know that `Simulator` stores an open `Display` instance.
        unsafe { x11::xlib::XCloseDisplay(self.display) };
    }
}
