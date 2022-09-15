use crate::Key;

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

    /// Sends a fake key press event to the top-level window.
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        let code = keysym_to_keycode(self, super::utils::key_to_keysym(key));
        if fake_key_event(self, code, true) == x11::xlib::False {
            return Err(super::Error::UnexpectedError);
        }

        flush(self);
        Ok(())
    }

    /// Sends a fake key release event to the top-level window.
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        let code = keysym_to_keycode(self, super::utils::key_to_keysym(key));
        if fake_key_event(self, code, false) == x11::xlib::False {
            return Err(super::Error::UnexpectedError);
        }

        flush(self);
        Ok(())
    }

    /// Sends a fake keystroke event to the top-level window.
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        let code = keysym_to_keycode(self, super::utils::key_to_keysym(key));

        if fake_key_event(self, code, true) == x11::xlib::False {
            return Err(super::Error::UnexpectedError);
        }

        if fake_key_event(self, code, false) == x11::xlib::False {
            return Err(super::Error::UnexpectedError);
        }

        flush(self);
        Ok(())
    }
}

impl Drop for Simulator {
    #[inline]
    fn drop(&mut self) {
        // Safety:
        //  By invariant, we know that `Simulator` stores an open `Display` instance.
        unsafe { x11::xlib::XCloseDisplay(self.display) };
    }
}

/// Converts a [`x11::xlib::KeySym`] into a [`x11::xlib::KeyCode`].
///
/// This function is wrapper around the [`x11::xlib::XKeysymToKeycode`] function.
#[inline]
fn keysym_to_keycode(sim: &Simulator, keysym: x11::xlib::KeySym) -> x11::xlib::KeyCode {
    // Safety:
    //  `Simulator` always holds a valid `Display`.
    unsafe { x11::xlib::XKeysymToKeycode(sim.display, keysym) }
}

/// Simulates a key event.
///
/// This function is wrapper around the [`x11::xtest::XTestFakeKeyEvent`] function.
#[inline]
fn fake_key_event(
    sim: &Simulator,
    keycode: x11::xlib::KeyCode,
    is_press: bool,
) -> x11::xlib::Status {
    // Safety:
    //  `Simulator` always holds a valid `Display`.
    unsafe { x11::xtest::XTestFakeKeyEvent(sim.display, keycode as _, is_press as _, 0) }
}

/// Flushes the output buffer.
///
/// This function is a wrapper around the [`x11::xlib::XFlush`] function.
#[inline]
fn flush(sim: &Simulator) {
    // Safety:
    //  `Simulator` always holds a valid `Display`.

    // I don't understand whether we are supposed to user the return value of `XFlush`... The man
    // never talks about potential errors, and some pages even omit that return type in the
    // function's signature.
    unsafe { x11::xlib::XFlush(sim.display) };
}
