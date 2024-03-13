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
        Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_BACK, VK_CAPITAL, VK_PACKET},
        WindowsAndMessaging::*,
    },
};

#[no_mangle]
pub unsafe extern "system" fn wh_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        let msg: &MSG = transmute(l_param);

        // TODO: add repeat check
        match msg.message {
            WM_KEYDOWN => 'keydown: {
                let msg_vk = VIRTUAL_KEY {
                    0: msg.wParam.0 as u16,
                };
                // ignore our input, Caps Lock
                if msg_vk == VK_PACKET || msg_vk == VK_BACK || msg_vk == VK_CAPITAL {
                    break 'keydown;
                }

                accent::update_input_state(&msg_vk);

                if let None = data::AccentKey::from_vk(&msg_vk) {
                    break 'keydown;
                }

                let (key, index) = accent::get_input_state().unwrap();
                dbg!(key, index);
                let is_capital = GetKeyState(VK_CAPITAL.0.into()) & 0x0001 != 0;

                accent::send_vk_back();
                accent::send_char(get_accent(key, is_capital, index));
            }
            // TODO: reset INPUT_STATE on focus change/"control" key press
            _ => (),
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
