// lib.rs
mod accent;
mod data;

#[cfg(test)]
mod tests;

use std::mem::transmute;
use windows::Win32::{
    Foundation::*,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

#[no_mangle]
pub unsafe extern "system" fn wh_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        let msg: &MSG = transmute(l_param);

        let current_key = match data::AccentKey::from_msg(&msg) {
            Some(val) => val,
            None => return CallNextHookEx(None, code, w_param, l_param),
        };

        let current_key = current_key
            .vk()
            .expect("current_key must be mapped to vk due to previous check");

        if current_key != VK_PACKET && current_key != VK_BACK {
            // TODO: add more logic here
            use data::*;
            accent::send_char(get_accent(AccentKey::O, true, 1));
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
