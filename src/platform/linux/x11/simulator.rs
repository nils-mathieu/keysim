use x11::{keysym, xlib};

use super::sys::OpenDisplay;
use super::utils;
use crate::{Button, Key};

/// The simulator used when using the X11 window manager.
pub struct Simulator {
    /// The display used that's being used to simulate keystrokes.
    display: OpenDisplay,

    /// Whether the current X11 server supports the "XTEST" extension.
    ///
    /// The XTEST extension is prefered because events that are sent through it cannot be
    /// distinguished from regular user events.
    supports_xtest: bool,
}

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    pub fn new() -> Result<Self, super::Error> {
        let display = OpenDisplay::open()?;
        let supports_xtest = display.xtest_query_extension();

        Ok(Self {
            display,
            supports_xtest,
        })
    }

    /// Sends a fake key press event to the top-level window.
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        let keycode = self
            .display
            .keysym_to_keycode(utils::key_to_x11(key))
            .ok_or(super::Error::UnsupportedKey(key))?;

        if self.supports_xtest {
            self.display.xtest_fake_key_event(keycode as _, true, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display.send_key_event(window, keycode as _, 0, true)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake key release event to the top-level window.
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        let keycode = self
            .display
            .keysym_to_keycode(utils::key_to_x11(key))
            .ok_or(super::Error::UnsupportedKey(key))?;

        if self.supports_xtest {
            self.display.xtest_fake_key_event(keycode as _, false, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display
                .send_key_event(window, keycode as _, 0, false)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake keystroke event to the top-level window.
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        let keycode = self
            .display
            .keysym_to_keycode(utils::key_to_x11(key))
            .ok_or(super::Error::UnsupportedKey(key))?;

        if self.supports_xtest {
            self.display.xtest_fake_key_event(keycode as _, true, 0)?;
            self.display.xtest_fake_key_event(keycode as _, false, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display.send_key_event(window, keycode as _, 0, true)?;
            self.display
                .send_key_event(window, keycode as _, 0, false)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button press to the top-level window.
    pub fn press_button(&self, button: Button) -> Result<(), super::Error> {
        let button = utils::button_to_x11(button);

        if self.supports_xtest {
            self.display.xtest_fake_button_event(button, true, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display.send_button_event(window, button, true)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button release to the top-level window.
    pub fn release_button(&self, button: Button) -> Result<(), super::Error> {
        let button = utils::button_to_x11(button);

        if self.supports_xtest {
            self.display.xtest_fake_button_event(button, false, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display.send_button_event(window, button, false)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button click to the top-level window.
    pub fn send_button(&self, button: Button) -> Result<(), super::Error> {
        let button = utils::button_to_x11(button);

        if self.supports_xtest {
            self.display.xtest_fake_button_event(button, true, 0)?;
            self.display.xtest_fake_button_event(button, false, 0)?;
        } else {
            let window = self.display.get_input_focus()?;
            self.display.send_button_event(window, button, true)?;
            self.display.send_button_event(window, button, false)?;
        }
        self.display.flush()?;
        Ok(())
    }

    /// Sends a unicode code-point using the "XTEST" extension.
    fn _send_char_xtest(&self, c: char) -> Result<(), super::Error> {
        let (keysym, shift) = utils::char_to_x11(c).ok_or(super::Error::UnsupportedChar(c))?;

        let keycode = self
            .display
            .keysym_to_keycode(keysym)
            .ok_or(super::Error::UnsupportedChar(c))?;
        let mut shift_keycode = 0;

        if shift {
            shift_keycode = self
                .display
                .keysym_to_keycode(keysym::XK_Shift_L as _)
                .ok_or(super::Error::UnsupportedChar(c))?;
            self.display
                .xtest_fake_key_event(shift_keycode as _, true, 0)?;
        }

        self.display.xtest_fake_key_event(keycode as _, true, 0)?;
        self.display.xtest_fake_key_event(keycode as _, false, 0)?;

        if shift {
            self.display
                .xtest_fake_key_event(shift_keycode as _, false, 0)?;
        }

        Ok(())
    }

    /// Sends a unicode code-point.
    fn _send_char(&self, window: xlib::Window, c: char) -> Result<(), super::Error> {
        let (keysym, shift) = utils::char_to_x11(c).ok_or(super::Error::UnsupportedChar(c))?;

        let keycode = self
            .display
            .keysym_to_keycode(keysym)
            .ok_or(super::Error::UnsupportedChar(c))?;
        let state = if shift { xlib::ShiftMask } else { 0 };
        self.display
            .send_key_event(window, keycode as _, state, true)?;
        self.display
            .send_key_event(window, keycode as _, state, false)?;

        Ok(())
    }

    /// Sends a specific unicode code-point.
    pub fn send_char(&self, c: char) -> Result<(), super::Error> {
        if self.supports_xtest {
            self._send_char_xtest(c)?;
        } else {
            let window = self.display.get_input_focus()?;
            self._send_char(window, c)?;
        }

        self.display.flush()?;
        Ok(())
    }

    /// Sends a collection of characters.
    pub fn send_chars(&self, mut it: impl Iterator<Item = char>) -> Result<(), super::Error> {
        if self.supports_xtest {
            it.try_for_each(|c| self._send_char_xtest(c))?;
        } else {
            let window = self.display.get_input_focus()?;
            it.try_for_each(move |c| self._send_char(window, c))?;
        }

        self.display.flush()?;
        Ok(())
    }

    /// Sends a string.
    pub fn send_str(&self, s: &str) -> Result<(), super::Error> {
        self.send_chars(s.chars())
    }
}
