use crate::{Button, Key};

use super::{sys, utils};

/// The simulator on the Windows platform.
#[derive(Default)]
pub struct Simulator;

impl Simulator {
    /// Creates a new [`Simulator`] instance.
    pub fn new() -> Result<Self, super::Error> {
        Ok(Self)
    }

    /// Simulates a key press event.
    pub fn press_key(&self, key: Key) -> Result<(), super::Error> {
        let vk = utils::key_to_vk(key).ok_or(super::Error::UnsupportedKey(key))?;
        let input = utils::make_key_event(vk as _, true);
        sys::send_events(std::slice::from_ref(&input))
    }

    /// Simulates a key release event.
    pub fn release_key(&self, key: Key) -> Result<(), super::Error> {
        let vk = utils::key_to_vk(key).ok_or(super::Error::UnsupportedKey(key))?;
        let input = utils::make_key_event(vk as _, false);
        sys::send_events(std::slice::from_ref(&input))
    }

    /// Simulates a keystroke.
    pub fn send_key(&self, key: Key) -> Result<(), super::Error> {
        let vk = utils::key_to_vk(key).ok_or(super::Error::UnsupportedKey(key))?;
        let inputs = [
            utils::make_key_event(vk as _, true),
            utils::make_key_event(vk as _, false),
        ];
        sys::send_events(&inputs)
    }

    /// Simulates a key press event.
    pub fn press_button(&self, button: Button) -> Result<(), super::Error> {
        let input = utils::make_button_event(button, true);
        sys::send_events(std::slice::from_ref(&input))
    }

    /// Simulates a key release event.
    pub fn release_button(&self, button: Button) -> Result<(), super::Error> {
        let input = utils::make_button_event(button, false);
        sys::send_events(std::slice::from_ref(&input))
    }

    /// Simulates a keystroke.
    pub fn send_button(&self, button: Button) -> Result<(), super::Error> {
        let inputs = [
            utils::make_button_event(button, true),
            utils::make_button_event(button, false),
        ];
        sys::send_events(&inputs)
    }

    /// Sends a specific unicode code-point.
    pub fn send_char(&self, c: char) -> Result<(), super::Error> {
        let inputs = [
            utils::make_char_event(c, true).ok_or(super::Error::UnsupportedChar(c))?,
            utils::make_char_event(c, false).ok_or(super::Error::UnsupportedChar(c))?,
        ];
        sys::send_events(&inputs)
    }

    /// Sends a collection of characters.
    pub fn send_chars(&self, mut it: impl Iterator<Item = char>) -> Result<(), super::Error> {
        it.try_for_each(|c| self.send_char(c))
    }

    /// Types a string.
    pub fn send_str(&self, s: &str) -> Result<(), super::Error> {
        self.send_chars(s.chars())
    }
}
