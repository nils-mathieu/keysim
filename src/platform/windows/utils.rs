//! Defines utility functions.

use std::os::raw::c_int;

use winapi::shared::minwindef;
use winapi::um::winuser;

use crate::{Button, Key};

/// Converts a [`Key`] into a Windows virtual-key code.
pub fn key_to_vk(key: Key) -> Option<c_int> {
    match key {
        Key::A => Some(b'A' as c_int),
        Key::B => Some(b'B' as c_int),
        Key::C => Some(b'C' as c_int),
        Key::D => Some(b'D' as c_int),
        Key::E => Some(b'E' as c_int),
        Key::F => Some(b'F' as c_int),
        Key::G => Some(b'G' as c_int),
        Key::H => Some(b'H' as c_int),
        Key::I => Some(b'I' as c_int),
        Key::J => Some(b'J' as c_int),
        Key::K => Some(b'K' as c_int),
        Key::L => Some(b'L' as c_int),
        Key::M => Some(b'M' as c_int),
        Key::N => Some(b'N' as c_int),
        Key::O => Some(b'O' as c_int),
        Key::P => Some(b'P' as c_int),
        Key::Q => Some(b'Q' as c_int),
        Key::R => Some(b'R' as c_int),
        Key::S => Some(b'S' as c_int),
        Key::T => Some(b'T' as c_int),
        Key::U => Some(b'U' as c_int),
        Key::V => Some(b'V' as c_int),
        Key::W => Some(b'W' as c_int),
        Key::X => Some(b'X' as c_int),
        Key::Y => Some(b'Y' as c_int),
        Key::Z => Some(b'Z' as c_int),
        Key::F1 => Some(winuser::VK_F1),
        Key::F2 => Some(winuser::VK_F2),
        Key::F3 => Some(winuser::VK_F3),
        Key::F4 => Some(winuser::VK_F4),
        Key::F5 => Some(winuser::VK_F5),
        Key::F6 => Some(winuser::VK_F6),
        Key::F7 => Some(winuser::VK_F7),
        Key::F8 => Some(winuser::VK_F8),
        Key::F9 => Some(winuser::VK_F9),
        Key::F10 => Some(winuser::VK_F10),
        Key::F11 => Some(winuser::VK_F11),
        Key::F12 => Some(winuser::VK_F12),
        Key::F13 => Some(winuser::VK_F13),
        Key::F14 => Some(winuser::VK_F14),
        Key::F15 => Some(winuser::VK_F15),
        Key::F16 => Some(winuser::VK_F16),
        Key::F17 => Some(winuser::VK_F17),
        Key::F18 => Some(winuser::VK_F18),
        Key::F19 => Some(winuser::VK_F19),
        Key::F20 => Some(winuser::VK_F20),
        Key::F21 => Some(winuser::VK_F21),
        Key::F22 => Some(winuser::VK_F22),
        Key::F23 => Some(winuser::VK_F23),
        Key::F24 => Some(winuser::VK_F24),
        Key::Zero => Some(b'0' as c_int),
        Key::One => Some(b'1' as c_int),
        Key::Two => Some(b'2' as c_int),
        Key::Three => Some(b'3' as c_int),
        Key::Four => Some(b'4' as c_int),
        Key::Five => Some(b'5' as c_int),
        Key::Six => Some(b'6' as c_int),
        Key::Seven => Some(b'7' as c_int),
        Key::Eight => Some(b'8' as c_int),
        Key::Nine => Some(b'9' as c_int),
        Key::Escape => Some(winuser::VK_ESCAPE),
        Key::Tab => Some(winuser::VK_TAB),
        Key::CapsLock => Some(winuser::VK_CAPITAL),
        Key::LeftShift => Some(winuser::VK_LSHIFT),
        Key::LeftControl => Some(winuser::VK_LCONTROL),
        Key::LeftAlt => Some(winuser::VK_LMENU),
        Key::LeftMeta => Some(winuser::VK_LWIN),
        Key::Space => Some(winuser::VK_SPACE),
        Key::RightMeta => Some(winuser::VK_RWIN),
        Key::RightAlt => Some(winuser::VK_RMENU),
        Key::RightControl => Some(winuser::VK_RCONTROL),
        Key::RightShift => Some(winuser::VK_RSHIFT),
        Key::Enter => Some(winuser::VK_RETURN),
        Key::Backspace => Some(winuser::VK_BACK),
        Key::Insert => Some(winuser::VK_INSERT),
        Key::Delete => Some(winuser::VK_DELETE),
        Key::Home => Some(winuser::VK_HOME),
        Key::End => Some(winuser::VK_END),
        Key::PageUp => Some(winuser::VK_PRIOR),
        Key::PageDown => Some(winuser::VK_NEXT),
        Key::ArrowUp => Some(winuser::VK_UP),
        Key::ArrowDown => Some(winuser::VK_DOWN),
        Key::ArrowLeft => Some(winuser::VK_LEFT),
        Key::ArrowRight => Some(winuser::VK_RIGHT),
        Key::NumLock => Some(winuser::VK_NUMLOCK),
        Key::NumpadDivide => Some(winuser::VK_DIVIDE),
        Key::NumpadEqual => None,
        Key::NumpadMultiply => Some(winuser::VK_MULTIPLY),
        Key::NumpadAdd => Some(winuser::VK_ADD),
        Key::NumpadEnter => Some(winuser::VK_RETURN),
        Key::NumpadDecimal => Some(winuser::VK_DECIMAL),
        Key::Numpad0 => Some(winuser::VK_NUMPAD0),
        Key::Numpad1 => Some(winuser::VK_NUMPAD1),
        Key::Numpad2 => Some(winuser::VK_NUMPAD2),
        Key::Numpad3 => Some(winuser::VK_NUMPAD3),
        Key::Numpad4 => Some(winuser::VK_NUMPAD4),
        Key::Numpad5 => Some(winuser::VK_NUMPAD5),
        Key::Numpad6 => Some(winuser::VK_NUMPAD6),
        Key::Numpad7 => Some(winuser::VK_NUMPAD7),
        Key::Numpad8 => Some(winuser::VK_NUMPAD8),
        Key::Numpad9 => Some(winuser::VK_NUMPAD9),
        Key::VolumeUp => Some(winuser::VK_VOLUME_UP),
        Key::VolumeDown => Some(winuser::VK_VOLUME_DOWN),
        Key::VolumeMute => Some(winuser::VK_VOLUME_MUTE),
        Key::MediaNext => Some(winuser::VK_MEDIA_NEXT_TRACK),
        Key::MediaPrevious => Some(winuser::VK_MEDIA_PREV_TRACK),
        Key::MediaStop => Some(winuser::VK_MEDIA_STOP),
        Key::MediaPause => Some(winuser::VK_MEDIA_PLAY_PAUSE),
    }
}

/// Creates a new [`winuser::INPUT`] instance, filled with zeros.
#[inline]
fn zeroed_input() -> winuser::INPUT {
    // Safety:
    //  `INPUT` is made only of primitive types.
    unsafe { std::mem::zeroed() }
}

/// Creates a [`winuser::INPUT`] instance that simulates a key press/release.
pub fn make_key_event(vk: minwindef::WORD, press: bool) -> winuser::INPUT {
    let mut ret = zeroed_input();
    ret.type_ = winuser::INPUT_KEYBOARD;

    let kbd = unsafe { ret.u.ki_mut() };

    kbd.wVk = vk;
    kbd.wScan = 0;

    if !press {
        kbd.dwFlags |= winuser::KEYEVENTF_KEYUP;
    }

    ret
}

/// Creates a [`winuser::INPUT`] instance that simulates a character being pressed/released.
pub fn make_char_event(c: char, press: bool) -> Option<winuser::INPUT> {
    let mut ret = zeroed_input();
    ret.type_ = winuser::INPUT_KEYBOARD;

    let kbd = unsafe { ret.u.ki_mut() };

    kbd.wScan = (c as u32).try_into().ok()?;
    kbd.dwFlags = winuser::KEYEVENTF_UNICODE;

    if !press {
        kbd.dwFlags |= winuser::KEYEVENTF_KEYUP;
    }

    Some(ret)
}

/// Creates a [`winuser::INPUT`] instance that simulates a mouse button being pressed/released.
pub fn make_button_event(button: Button, press: bool) -> winuser::INPUT {
    let mut ret = zeroed_input();
    ret.type_ = winuser::INPUT_MOUSE;

    let mouse = unsafe { ret.u.mi_mut() };

    match (button, press) {
        (Button::Left, false) => mouse.dwFlags = winuser::MOUSEEVENTF_LEFTUP,
        (Button::Left, true) => mouse.dwFlags = winuser::MOUSEEVENTF_LEFTDOWN,
        (Button::Right, false) => mouse.dwFlags = winuser::MOUSEEVENTF_RIGHTUP,
        (Button::Right, true) => mouse.dwFlags = winuser::MOUSEEVENTF_RIGHTDOWN,
        (Button::Middle, false) => mouse.dwFlags = winuser::MOUSEEVENTF_MIDDLEUP,
        (Button::Middle, true) => mouse.dwFlags = winuser::MOUSEEVENTF_MIDDLEDOWN,
        (Button::Extra(n), false) => {
            mouse.mouseData = n as _;
            mouse.dwFlags = winuser::MOUSEEVENTF_XUP
        }
        (Button::Extra(n), true) => {
            mouse.mouseData = n as _;
            mouse.dwFlags = winuser::MOUSEEVENTF_XDOWN
        }
    }

    ret
}
