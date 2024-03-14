// hook.rs

mod accent;
mod data;
#[cfg(test)]
mod tests;

use windows::Win32::{
    Foundation::*,
    UI::{
        Input::KeyboardAndMouse::{
            GetAsyncKeyState, GetKeyState, VIRTUAL_KEY, VK_BACK, VK_CAPITAL, VK_LCONTROL, VK_PACKET,
        },
        WindowsAndMessaging::*,
    },
};

// Install hook
pub fn setup_hook() -> HHOOK {
    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(callback), None, 0);

        match hook {
            Err(_) => panic!("Failed to install hook!"),
            Ok(hhk) => {
                println!("Hook was installed successefully.");
                hhk
            }
        }
    }
}

// Uninstall hook
pub fn remove_hook(hook: HHOOK) {
    unsafe {
        let result = UnhookWindowsHookEx(hook);

        match result {
            Err(_) => panic!("Failed to remove hook!"),
            Ok(_) => println!("Hook was removed successefully."),
        }
    }
}

// Main hook logic
unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        let msg: &KBDLLHOOKSTRUCT = std::mem::transmute(l_param);

        match w_param.0 as u32 {
            WM_KEYDOWN => 'keydown: {
                let msg_vk = VIRTUAL_KEY {
                    0: msg.vkCode as u16,
                };

                // Left control as control key
                // Note: conflict with defaulth shortcuts such as ctrl + c, ctrl + v etc
                let control = GetAsyncKeyState(VK_LCONTROL.0.into()) & 0x8000u16 as i16 != 0;
                dbg!(control);

                // ignore our input, Caps Lock
                if msg_vk == VK_PACKET || msg_vk == VK_BACK || msg_vk == VK_CAPITAL || !control {
                    break 'keydown;
                }

                accent::update_input_state(&msg_vk);

                if let None = data::AccentKey::from_vk(&msg_vk) {
                    break 'keydown;
                }

                let (key, index) = accent::get_input_state().unwrap();
                let is_capital = GetKeyState(VK_CAPITAL.0.into()) & 0x0001 != 0;

                // accent::send_vk_back();
                accent::send_char(data::get_accent(key, is_capital, index));

                return LRESULT(1);
            }
            WM_SYSKEYDOWN => {
                // TODO: implement control logic here
            }
            _ => (),
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
