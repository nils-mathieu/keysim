use super::sys::OpenDisplay;
use super::utils;
use crate::{Button, Key};

/// The simulator used when using the X11 window manager.
pub struct Simulator {
    /// The display used that's being used to simulate keystrokes.
    display: OpenDisplay,
}

// TODO:
//  Figure out wether the `Simulator` type should be thread safe or not.

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    pub fn new() -> Result<Self, super::Error> {
        Ok(Self {
            display: OpenDisplay::open()?,
        })
    }

    /// Sends a fake key press event to the top-level window.
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let keycode = self.display.keysym_to_keycode(utils::key_to_x11(key));
        self.display.send_key_event(window, keycode as _, true)?;
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake key release event to the top-level window.
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let keycode = self.display.keysym_to_keycode(utils::key_to_x11(key));
        self.display.send_key_event(window, keycode as _, false)?;
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake keystroke event to the top-level window.
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let keycode = self.display.keysym_to_keycode(utils::key_to_x11(key));
        self.display.send_key_event(window, keycode as _, true)?;
        self.display.send_key_event(window, keycode as _, false)?;
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button press to the top-level window.
    pub fn press_button(&self, button: Button) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let button = utils::button_to_x11(button);
        self.display.send_button_event(window, button, true)?;
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button release to the top-level window.
    pub fn release_button(&self, button: Button) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let button = utils::button_to_x11(button);
        self.display.send_button_event(window, button, false)?;
        self.display.flush()?;
        Ok(())
    }

    /// Sends a fake button click to the top-level window.
    pub fn send_button(&self, button: Button) -> Result<(), super::Error> {
        let window = self.display.get_input_focus()?;
        let button = utils::button_to_x11(button);
        self.display.send_button_event(window, button, true)?;
        self.display.send_button_event(window, button, false)?;
        self.display.flush()?;
        Ok(())
    }
}
