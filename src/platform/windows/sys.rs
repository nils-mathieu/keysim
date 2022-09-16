//! This module wraps the Windows API behind safe function calls.

use winapi::um::winuser;

/// Sends the provided events.
pub fn send_events(events: &[winuser::INPUT]) -> Result<(), super::Error> {
    let mut c = events.len() as u32;
    let mut p = events.as_ptr() as *mut winuser::INPUT;
    let s = std::mem::size_of::<winuser::INPUT>() as i32;

    while c != 0 {
        // Safety:
        //  Those parameters have been extracted from a regular Rust reference, ensuring that hey
        //  remain valid for the lifetime of the present function.
        let inserted = unsafe { winuser::SendInput(c, p, s) };

        if inserted == 0 {
            return Err(super::Error::Blocked);
        }

        c -= inserted;

        // Safety:
        //  The `SendInput` function cannot insert more elements than provided. This operation
        //  cannot overflow the original slice.
        p = unsafe { p.add(inserted as usize) };
    }

    Ok(())
}
