//! This module provides utility functions to work with the X11 API.

use std::os::raw::c_uint;

use x11::{keysym, xlib};

use crate::{Button, Key};

/// Converts a [`Key`] into a [`xlib::KeySym`].
pub fn key_to_x11(key: Key) -> xlib::KeySym {
    match key {
        Key::A => keysym::XK_A as xlib::KeySym,
        Key::B => keysym::XK_B as xlib::KeySym,
        Key::C => keysym::XK_C as xlib::KeySym,
        Key::D => keysym::XK_D as xlib::KeySym,
        Key::E => keysym::XK_E as xlib::KeySym,
        Key::F => keysym::XK_F as xlib::KeySym,
        Key::G => keysym::XK_G as xlib::KeySym,
        Key::H => keysym::XK_H as xlib::KeySym,
        Key::I => keysym::XK_I as xlib::KeySym,
        Key::J => keysym::XK_J as xlib::KeySym,
        Key::K => keysym::XK_K as xlib::KeySym,
        Key::L => keysym::XK_L as xlib::KeySym,
        Key::M => keysym::XK_M as xlib::KeySym,
        Key::N => keysym::XK_N as xlib::KeySym,
        Key::O => keysym::XK_O as xlib::KeySym,
        Key::P => keysym::XK_P as xlib::KeySym,
        Key::Q => keysym::XK_Q as xlib::KeySym,
        Key::R => keysym::XK_R as xlib::KeySym,
        Key::S => keysym::XK_S as xlib::KeySym,
        Key::T => keysym::XK_T as xlib::KeySym,
        Key::U => keysym::XK_U as xlib::KeySym,
        Key::V => keysym::XK_V as xlib::KeySym,
        Key::W => keysym::XK_W as xlib::KeySym,
        Key::X => keysym::XK_X as xlib::KeySym,
        Key::Y => keysym::XK_Y as xlib::KeySym,
        Key::Z => keysym::XK_Z as xlib::KeySym,
        Key::F1 => keysym::XK_F1 as xlib::KeySym,
        Key::F2 => keysym::XK_F2 as xlib::KeySym,
        Key::F3 => keysym::XK_F3 as xlib::KeySym,
        Key::F4 => keysym::XK_F4 as xlib::KeySym,
        Key::F5 => keysym::XK_F5 as xlib::KeySym,
        Key::F6 => keysym::XK_F6 as xlib::KeySym,
        Key::F7 => keysym::XK_F7 as xlib::KeySym,
        Key::F8 => keysym::XK_F8 as xlib::KeySym,
        Key::F9 => keysym::XK_F9 as xlib::KeySym,
        Key::F10 => keysym::XK_F10 as xlib::KeySym,
        Key::F11 => keysym::XK_F11 as xlib::KeySym,
        Key::F12 => keysym::XK_F12 as xlib::KeySym,
        Key::F13 => keysym::XK_F13 as xlib::KeySym,
        Key::F14 => keysym::XK_F14 as xlib::KeySym,
        Key::F15 => keysym::XK_F15 as xlib::KeySym,
        Key::F16 => keysym::XK_F16 as xlib::KeySym,
        Key::F17 => keysym::XK_F17 as xlib::KeySym,
        Key::F18 => keysym::XK_F18 as xlib::KeySym,
        Key::F19 => keysym::XK_F19 as xlib::KeySym,
        Key::F20 => keysym::XK_F20 as xlib::KeySym,
        Key::F21 => keysym::XK_F21 as xlib::KeySym,
        Key::F22 => keysym::XK_F22 as xlib::KeySym,
        Key::F23 => keysym::XK_F23 as xlib::KeySym,
        Key::F24 => keysym::XK_F24 as xlib::KeySym,
        Key::Zero => keysym::XK_0 as xlib::KeySym,
        Key::One => keysym::XK_1 as xlib::KeySym,
        Key::Two => keysym::XK_2 as xlib::KeySym,
        Key::Three => keysym::XK_3 as xlib::KeySym,
        Key::Four => keysym::XK_4 as xlib::KeySym,
        Key::Five => keysym::XK_5 as xlib::KeySym,
        Key::Six => keysym::XK_6 as xlib::KeySym,
        Key::Seven => keysym::XK_7 as xlib::KeySym,
        Key::Eight => keysym::XK_8 as xlib::KeySym,
        Key::Nine => keysym::XK_9 as xlib::KeySym,
        Key::Escape => keysym::XK_Escape as xlib::KeySym,
        Key::Tab => keysym::XK_Tab as xlib::KeySym,
        Key::CapsLock => keysym::XK_Caps_Lock as xlib::KeySym,
        Key::LeftShift => keysym::XK_Shift_L as xlib::KeySym,
        Key::LeftControl => keysym::XK_Control_L as xlib::KeySym,
        Key::LeftAlt => keysym::XK_Alt_L as xlib::KeySym,
        Key::LeftMeta => keysym::XK_Meta_L as xlib::KeySym,
        Key::Space => keysym::XK_space as xlib::KeySym,
        Key::RightMeta => keysym::XK_Meta_R as xlib::KeySym,
        Key::RightAlt => keysym::XK_Alt_R as xlib::KeySym,
        Key::RightControl => keysym::XK_Control_R as xlib::KeySym,
        Key::RightShift => keysym::XK_Shift_R as xlib::KeySym,
        Key::Enter => keysym::XK_Return as xlib::KeySym,
        Key::Backspace => keysym::XK_BackSpace as xlib::KeySym,
        Key::Insert => keysym::XK_Insert as xlib::KeySym,
        Key::Delete => keysym::XK_Delete as xlib::KeySym,
        Key::Home => keysym::XK_Home as xlib::KeySym,
        Key::End => keysym::XK_End as xlib::KeySym,
        Key::PageUp => keysym::XK_Page_Up as xlib::KeySym,
        Key::PageDown => keysym::XK_Page_Down as xlib::KeySym,
        Key::ArrowUp => keysym::XK_Up as xlib::KeySym,
        Key::ArrowDown => keysym::XK_Down as xlib::KeySym,
        Key::ArrowLeft => keysym::XK_Left as xlib::KeySym,
        Key::ArrowRight => keysym::XK_Right as xlib::KeySym,
        Key::NumLock => keysym::XK_Num_Lock as xlib::KeySym,
        Key::NumpadEqual => keysym::XK_KP_Equal as xlib::KeySym,
        Key::NumpadDivide => keysym::XK_KP_Divide as xlib::KeySym,
        Key::NumpadMultiply => keysym::XK_KP_Multiply as xlib::KeySym,
        Key::NumpadAdd => keysym::XK_KP_Add as xlib::KeySym,
        Key::NumpadEnter => keysym::XK_KP_Enter as xlib::KeySym,
        Key::NumpadDecimal => keysym::XK_KP_Decimal as xlib::KeySym,
        Key::Numpad0 => keysym::XK_KP_0 as xlib::KeySym,
        Key::Numpad1 => keysym::XK_KP_1 as xlib::KeySym,
        Key::Numpad2 => keysym::XK_KP_2 as xlib::KeySym,
        Key::Numpad3 => keysym::XK_KP_3 as xlib::KeySym,
        Key::Numpad4 => keysym::XK_KP_4 as xlib::KeySym,
        Key::Numpad5 => keysym::XK_KP_5 as xlib::KeySym,
        Key::Numpad6 => keysym::XK_KP_6 as xlib::KeySym,
        Key::Numpad7 => keysym::XK_KP_7 as xlib::KeySym,
        Key::Numpad8 => keysym::XK_KP_8 as xlib::KeySym,
        Key::Numpad9 => keysym::XK_KP_9 as xlib::KeySym,
        Key::VolumeUp => keysym::XF86XK_AudioRaiseVolume as xlib::KeySym,
        Key::VolumeDown => keysym::XF86XK_AudioLowerVolume as xlib::KeySym,
        Key::VolumeMute => keysym::XF86XK_AudioMute as xlib::KeySym,
        Key::MediaNext => keysym::XF86XK_AudioNext as xlib::KeySym,
        Key::MediaPrevious => keysym::XF86XK_AudioPrev as xlib::KeySym,
        Key::MediaStop => keysym::XF86XK_AudioStop as xlib::KeySym,
        Key::MediaPause => keysym::XF86XK_AudioPause as xlib::KeySym,
    }
}

/// Converts a [`Button`] instance into a [`c_uint`] understood by X11.
pub fn button_to_x11(button: Button) -> c_uint {
    match button {
        Button::Left => 1,
        Button::Middle => 2,
        Button::Right => 3,
        Button::Extra(0) => 8,
        Button::Extra(1) => 9,
        Button::Extra(_) => 0,
    }
}

/// Converts the given character into a [`xlib::KeySym`] instance, if possible.
///
/// The `bool` indicates whether the shift key should be pressed.
pub fn char_to_x11(c: char) -> Option<(xlib::KeySym, bool)> {
    match c {
        'a' => Some((keysym::XK_a as xlib::KeySym, false)),
        'b' => Some((keysym::XK_b as xlib::KeySym, false)),
        'c' => Some((keysym::XK_c as xlib::KeySym, false)),
        'd' => Some((keysym::XK_d as xlib::KeySym, false)),
        'e' => Some((keysym::XK_e as xlib::KeySym, false)),
        'f' => Some((keysym::XK_f as xlib::KeySym, false)),
        'g' => Some((keysym::XK_g as xlib::KeySym, false)),
        'h' => Some((keysym::XK_h as xlib::KeySym, false)),
        'i' => Some((keysym::XK_i as xlib::KeySym, false)),
        'j' => Some((keysym::XK_j as xlib::KeySym, false)),
        'k' => Some((keysym::XK_k as xlib::KeySym, false)),
        'l' => Some((keysym::XK_l as xlib::KeySym, false)),
        'm' => Some((keysym::XK_m as xlib::KeySym, false)),
        'n' => Some((keysym::XK_n as xlib::KeySym, false)),
        'o' => Some((keysym::XK_o as xlib::KeySym, false)),
        'p' => Some((keysym::XK_p as xlib::KeySym, false)),
        'q' => Some((keysym::XK_q as xlib::KeySym, false)),
        'r' => Some((keysym::XK_r as xlib::KeySym, false)),
        's' => Some((keysym::XK_s as xlib::KeySym, false)),
        't' => Some((keysym::XK_t as xlib::KeySym, false)),
        'u' => Some((keysym::XK_u as xlib::KeySym, false)),
        'v' => Some((keysym::XK_v as xlib::KeySym, false)),
        'w' => Some((keysym::XK_w as xlib::KeySym, false)),
        'x' => Some((keysym::XK_x as xlib::KeySym, false)),
        'y' => Some((keysym::XK_y as xlib::KeySym, false)),
        'z' => Some((keysym::XK_z as xlib::KeySym, false)),
        'A' => Some((keysym::XK_A as xlib::KeySym, true)),
        'B' => Some((keysym::XK_B as xlib::KeySym, true)),
        'C' => Some((keysym::XK_C as xlib::KeySym, true)),
        'D' => Some((keysym::XK_D as xlib::KeySym, true)),
        'E' => Some((keysym::XK_E as xlib::KeySym, true)),
        'F' => Some((keysym::XK_F as xlib::KeySym, true)),
        'G' => Some((keysym::XK_G as xlib::KeySym, true)),
        'H' => Some((keysym::XK_H as xlib::KeySym, true)),
        'I' => Some((keysym::XK_I as xlib::KeySym, true)),
        'J' => Some((keysym::XK_J as xlib::KeySym, true)),
        'K' => Some((keysym::XK_K as xlib::KeySym, true)),
        'L' => Some((keysym::XK_L as xlib::KeySym, true)),
        'M' => Some((keysym::XK_M as xlib::KeySym, true)),
        'N' => Some((keysym::XK_N as xlib::KeySym, true)),
        'O' => Some((keysym::XK_O as xlib::KeySym, true)),
        'P' => Some((keysym::XK_P as xlib::KeySym, true)),
        'Q' => Some((keysym::XK_Q as xlib::KeySym, true)),
        'R' => Some((keysym::XK_R as xlib::KeySym, true)),
        'S' => Some((keysym::XK_S as xlib::KeySym, true)),
        'T' => Some((keysym::XK_T as xlib::KeySym, true)),
        'U' => Some((keysym::XK_U as xlib::KeySym, true)),
        'V' => Some((keysym::XK_V as xlib::KeySym, true)),
        'W' => Some((keysym::XK_W as xlib::KeySym, true)),
        'X' => Some((keysym::XK_X as xlib::KeySym, true)),
        'Y' => Some((keysym::XK_Y as xlib::KeySym, true)),
        'Z' => Some((keysym::XK_Z as xlib::KeySym, true)),
        '0' => Some((keysym::XK_0 as xlib::KeySym, false)),
        '1' => Some((keysym::XK_1 as xlib::KeySym, false)),
        '2' => Some((keysym::XK_2 as xlib::KeySym, false)),
        '3' => Some((keysym::XK_3 as xlib::KeySym, false)),
        '4' => Some((keysym::XK_4 as xlib::KeySym, false)),
        '5' => Some((keysym::XK_5 as xlib::KeySym, false)),
        '6' => Some((keysym::XK_6 as xlib::KeySym, false)),
        '7' => Some((keysym::XK_7 as xlib::KeySym, false)),
        '8' => Some((keysym::XK_8 as xlib::KeySym, false)),
        '9' => Some((keysym::XK_9 as xlib::KeySym, false)),
        ' ' => Some((keysym::XK_space as xlib::KeySym, false)),
        '!' => Some((keysym::XK_exclam as xlib::KeySym, true)),
        '"' => Some((keysym::XK_quotedbl as xlib::KeySym, true)),
        '#' => Some((keysym::XK_numbersign as xlib::KeySym, true)),
        '$' => Some((keysym::XK_dollar as xlib::KeySym, true)),
        '%' => Some((keysym::XK_percent as xlib::KeySym, true)),
        '&' => Some((keysym::XK_ampersand as xlib::KeySym, true)),
        '(' => Some((keysym::XK_parenleft as xlib::KeySym, true)),
        ')' => Some((keysym::XK_parenright as xlib::KeySym, true)),
        '*' => Some((keysym::XK_asterisk as xlib::KeySym, true)),
        '+' => Some((keysym::XK_plus as xlib::KeySym, true)),
        ',' => Some((keysym::XK_comma as xlib::KeySym, false)),
        '-' => Some((keysym::XK_minus as xlib::KeySym, false)),
        '.' => Some((keysym::XK_period as xlib::KeySym, false)),
        '/' => Some((keysym::XK_slash as xlib::KeySym, false)),
        ':' => Some((keysym::XK_colon as xlib::KeySym, true)),
        ';' => Some((keysym::XK_semicolon as xlib::KeySym, false)),
        '<' => Some((keysym::XK_less as xlib::KeySym, true)),
        '=' => Some((keysym::XK_equal as xlib::KeySym, false)),
        '>' => Some((keysym::XK_greater as xlib::KeySym, true)),
        '?' => Some((keysym::XK_question as xlib::KeySym, true)),
        '@' => Some((keysym::XK_at as xlib::KeySym, true)),
        '[' => Some((keysym::XK_bracketleft as xlib::KeySym, false)),
        '\'' => Some((keysym::XK_hyphen as xlib::KeySym, false)),
        '\\' => Some((keysym::XK_backslash as xlib::KeySym, false)),
        '\n' => Some((keysym::XK_Return as xlib::KeySym, false)),
        '\t' => Some((keysym::XK_Tab as xlib::KeySym, false)),
        ']' => Some((keysym::XK_bracketright as xlib::KeySym, false)),
        '^' => Some((keysym::XK_asciicircum as xlib::KeySym, true)),
        '_' => Some((keysym::XK_underscore as xlib::KeySym, true)),
        '`' => Some((keysym::XK_grave as xlib::KeySym, false)),
        '{' => Some((keysym::XK_braceleft as xlib::KeySym, true)),
        '|' => Some((keysym::XK_bar as xlib::KeySym, true)),
        '}' => Some((keysym::XK_braceright as xlib::KeySym, true)),
        '~' => Some((keysym::XK_asciitilde as xlib::KeySym, true)),
        _ => None,
    }
}
