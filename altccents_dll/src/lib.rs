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

        match msg.message as u32 {
            WM_KEYDOWN => {
                println!("Key down: {:?}", msg.wParam);

                if msg.wParam.0 != VK_PACKET.0.into() && msg.wParam.0 != VK_BACK.0.into() {
                    use data::*;
                    accent::send_char(get_accent(AccentKey::O, true, 1));
                }
            }
            _ => (),
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
