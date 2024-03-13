// lib.rs
mod accent;
mod data;

#[cfg(test)]
mod tests;

use data::*;
use std::mem::transmute;
use windows::Win32::{
    Foundation::*,
    UI::{
        Input::KeyboardAndMouse::{GetKeyState, VK_CAPITAL},
        WindowsAndMessaging::*,
    },
};

#[no_mangle]
pub unsafe extern "system" fn wh_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        let msg: &MSG = transmute(l_param);

        // TODO: add repeat check
        match msg.message {
            WM_KEYDOWN => {
                let current_key = match data::AccentKey::from_msg(&msg) {
                    Some(val) => val,
                    None => return CallNextHookEx(None, code, w_param, l_param),
                };

                let current_key = current_key
                    .vk()
                    .expect("current_key must be mapped to vk due to previous check");

                accent::update_input_state(&current_key);
                let (key, index) = accent::get_input_state().unwrap();
                dbg!(key, index);
                let is_capital = GetKeyState(VK_CAPITAL.0.into()) & 0x0001 != 0;

                accent::send_char(get_accent(key, is_capital, index));
                accent::enable_wm_char_capturer();
            }
            // TODO: catch system WM_CHAR message after accent::send_char was called
            WM_CHAR => {
                dbg!(accent::get_wm_char_capturer_state());
                if accent::get_wm_char_capturer_state() {
                    accent::disable_wm_char_capturer();
                    // return LRESULT(1);
                    // Somehow capture WM_CHAR here
                    // Use WH_CALLWNDPROC hook?
                }
            }
            _ => (),
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
