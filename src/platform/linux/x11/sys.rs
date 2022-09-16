//! Wraps the calls to the X11 API into safe function calls associated to the [`Display`] type.

use std::os::raw::c_uint;

use x11::xlib;

/// An open connection with the X server.
pub struct OpenDisplay {
    raw: *mut x11::xlib::Display,
}

impl OpenDisplay {
    /// Opens a connection with the X11 server.
    pub fn open() -> Result<Self, super::Error> {
        // Calling `XOpenDisplay` with a null pointer is always safe.
        let raw = unsafe { xlib::XOpenDisplay(std::ptr::null()) };

        if raw.is_null() {
            return Err(super::Error::OpenError);
        }

        Ok(Self { raw })
    }

    /// Returns the raw pointer owned by this instance of `Display`.
    ///
    /// The `[`xlib::XCloseDisplay`] function must not be called on the returned pointer! This
    /// function is not `unsafe` because calling [`xlib::XCloseDisplay`] is already unsafe, but this
    /// should be considered *unsafe*.
    #[inline]
    pub fn raw(&self) -> *mut x11::xlib::Display {
        self.raw
    }

    /// Wraps the [`xlib::XGetInputFocus`] function.
    #[inline]
    pub fn get_input_focus(&self) -> Result<xlib::Window, super::Error> {
        let mut window = 0;
        let mut revert_to = 0;

        // Safety:
        //  The `raw` field of `Display` is known to be valid, by invariant.
        let status = unsafe { xlib::XGetInputFocus(self.raw(), &mut window, &mut revert_to) };

        if status == xlib::False {
            Err(super::Error::UnexpectedError)
        } else {
            Ok(window)
        }
    }

    /// Wraps the [`xlib::XKeysymToKeycode`] function.
    #[inline]
    pub fn keysym_to_keycode(&self, keysym: xlib::KeySym) -> xlib::KeyCode {
        // Safety:
        //  The `raw` field of `Display` is known to be valid, by invariant.
        unsafe { xlib::XKeysymToKeycode(self.raw, keysym) }
    }

    /// Wraps the [`xlib::XFlush`] function.
    #[inline]
    pub fn flush(&self) -> Result<(), super::Error> {
        // It's not clear wether the `XFlush` function can ever fail. It is supposed to return
        // a `Status`, but no documentation ever mention that function failing, and some even omit
        // its return type.

        // Safety:
        //  The `raw` field of `Display` is known to be valid, by invariant.
        let status = unsafe { xlib::XFlush(self.raw) };

        if status == xlib::False {
            Err(super::Error::UnexpectedError)
        } else {
            Ok(())
        }
    }

    /// Sends a [`XKeyEvent`].
    ///
    /// This function assumes that `window` is a valid window ID, meaning that it returns
    /// [`UnexpectedError`] if the operation fails.
    ///
    /// This function wraps the [`xlib::XSendEvent`] function.
    ///
    /// [`UnexpectedError`]: super::Error::UnexpectedError
    #[inline]
    pub fn send_key_event(
        &self,
        window: xlib::Window,
        keycode: c_uint,
        press: bool,
    ) -> Result<(), super::Error> {
        let (type_, mask) = match press {
            true => (xlib::KeyPress, xlib::KeyPressMask),
            false => (xlib::KeyRelease, xlib::KeyReleaseMask),
        };

        let mut event = xlib::XEvent {
            key: xlib::XKeyEvent {
                display: self.raw,
                keycode,
                root: 0,
                same_screen: xlib::True,
                send_event: xlib::True,
                serial: 0,
                state: 0,
                subwindow: 0,
                time: xlib::CurrentTime,
                type_,
                window,
                x: 0,
                x_root: 0,
                y: 0,
                y_root: 0,
            },
        };

        let status = unsafe { xlib::XSendEvent(self.raw, window, xlib::True, mask, &mut event) };

        if status == xlib::False {
            Err(super::Error::UnexpectedError)
        } else {
            Ok(())
        }
    }

    /// Sends a [`XButtonEvent`].
    ///
    /// This function assumes that `window` is a valid window ID, meaning that it returns
    /// [`UnexpectedError`] if the operation fails.
    ///
    /// This function wraps the [`xlib::XSendEvent`] function.
    ///
    /// [`UnexpectedError`]: super::Error::UnexpectedError
    #[inline]
    pub fn send_button_event(
        &self,
        window: xlib::Window,
        button: c_uint,
        press: bool,
    ) -> Result<(), super::Error> {
        let (type_, mask) = match press {
            true => (xlib::ButtonPress, xlib::ButtonPressMask),
            false => (xlib::ButtonRelease, xlib::ButtonReleaseMask),
        };

        let mut event = xlib::XEvent {
            button: xlib::XButtonEvent {
                button,
                display: self.raw,
                root: 0,
                same_screen: xlib::True,
                send_event: xlib::True,
                serial: 0,
                state: 0,
                subwindow: 0,
                time: xlib::CurrentTime,
                type_,
                window,
                x: 0,
                x_root: 0,
                y: 0,
                y_root: 0,
            },
        };

        // Safety:
        //  The `raw` field of `Display` is known to be valid, by invariant.
        let status = unsafe { xlib::XSendEvent(self.raw, window, xlib::True, mask, &mut event) };

        if status == xlib::False {
            Err(super::Error::UnexpectedError)
        } else {
            Ok(())
        }
    }
}

impl Drop for OpenDisplay {
    #[inline]
    fn drop(&mut self) {
        // Safety:
        //  The `raw` field of `Display` is known to be valid, by invariant.
        unsafe { xlib::XCloseDisplay(self.raw) };
    }
}
