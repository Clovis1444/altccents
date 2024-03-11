// lib.rs
mod accent;
mod data;

#[cfg(test)]
mod tests;

use data::*;
use std::mem::transmute;
use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

use crate::accent::{get_input_state, update_input_state};

#[no_mangle]
pub unsafe extern "system" fn wh_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        let msg: &MSG = transmute(l_param);

        // TODO: check "shift"/"caps lock" state
        if let WM_KEYDOWN = msg.message {
            let current_key = match data::AccentKey::from_msg(&msg) {
                Some(val) => val,
                None => return CallNextHookEx(None, code, w_param, l_param),
            };

            let current_key = current_key
                .vk()
                .expect("current_key must be mapped to vk due to previous check");

            update_input_state(&current_key);
            let (key, index) = get_input_state().unwrap();
            dbg!(key, index);
            accent::send_char(get_accent(key, true, index));
        }
        // TODO: catch system WM_CHAR message after accent::send_char was called
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
