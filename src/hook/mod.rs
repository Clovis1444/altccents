// hook.rs

pub mod accent;
pub mod data;
#[cfg(test)]
mod tests;
mod timer;

use super::{config::*, session::PROGRAM_DATA, window};

use windows::Win32::{
    Foundation::*,
    UI::{
        Input::KeyboardAndMouse::{
            GetKeyState, VIRTUAL_KEY, VK_BACK, VK_CAPITAL, VK_LSHIFT, VK_PACKET, VK_RSHIFT,
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
    if code == HC_ACTION.try_into().unwrap() && PROGRAM_DATA.get_status() {
        let msg: &KBDLLHOOKSTRUCT = std::mem::transmute(l_param);

        match w_param.0 as u32 {
            WM_KEYDOWN => 'keydown: {
                let msg_vk = VIRTUAL_KEY {
                    0: msg.vkCode as u16,
                };

                let control = GetKeyState(CONTROL_KEY.0.into()) & 0x8000u16 as i16 != 0;

                // Redraw popup on shift/caps lock changes
                if control && (msg_vk == VK_CAPITAL || msg_vk == VK_LSHIFT || msg_vk == VK_RSHIFT) {
                    window::redraw();
                }

                // ignore CONTROL_KEY, Caps Lock, L an R Shift
                if msg_vk == VK_PACKET
                    || msg_vk == VK_BACK
                    || msg_vk == VK_CAPITAL
                    || msg_vk == VK_LSHIFT
                    || msg_vk == VK_RSHIFT
                    || !control
                {
                    break 'keydown;
                }

                // Send if: current key is not an accent OR current accent != previous accent
                'send_if_no_accent_or_other_accent: {
                    let previous_accent = match accent::get_input_state() {
                        Some((key, _)) => key,
                        None => break 'send_if_no_accent_or_other_accent,
                    };

                    match data::AccentKey::from_vk(&msg_vk) {
                        Some(current_accent) => {
                            if current_accent == previous_accent {
                                break 'send_if_no_accent_or_other_accent;
                            }
                        }
                        None => (),
                    };

                    accent::send_accent_and_kill_timer()
                }

                accent::update_input_state(&msg_vk);

                // Redraw popup
                window::redraw();

                // If current key is not accent key - send default
                if let None = data::AccentKey::from_vk(&msg_vk) {
                    break 'keydown;
                }

                return LRESULT(1);
            }
            WM_KEYUP => {
                let msg_vk = VIRTUAL_KEY {
                    0: msg.vkCode as u16,
                };

                match msg_vk {
                    CONTROL_KEY => {
                        accent::send_accent_and_kill_timer();

                        accent::reset_input_state();

                        // Redraw popup
                        window::redraw();
                    }
                    // Redraw popup if shift was released and CONTROL_KEY is pressed
                    VK_LSHIFT | VK_RSHIFT => {
                        let control = GetKeyState(CONTROL_KEY.0.into()) & 0x8000u16 as i16 != 0;
                        if control {
                            window::redraw();
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
