// hook.rs

pub mod accent;
pub mod data;
#[cfg(test)]
mod tests;
mod timer;

use super::{config::*, popup, session, session::PROGRAM_DATA, tray};

use windows::Win32::{
    Foundation::*,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
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

                let control = GetKeyState(CONTROL_KEY.0.into()) & 0x8000u16 as i16 != 0;

                // If win + shift + control_key -> change program state and break
                if msg_vk == CONTROL_KEY {
                    let win = GetKeyState(VK_LWIN.0.into()) & 0x8000u16 as i16 != 0
                        || GetKeyState(VK_RWIN.0.into()) & 0x8000u16 as i16 != 0;

                    let shift = GetKeyState(VK_LSHIFT.0.into()) & 0x8000u16 as i16 != 0
                        || GetKeyState(VK_RSHIFT.0.into()) & 0x8000u16 as i16 != 0;

                    if win && shift {
                        session::PROGRAM_DATA.change_status();
                        // Redraw tray icon
                        tray::update_tray_icon(&mut PROGRAM_DATA);
                        break 'keydown;
                    }
                }

                // If programm is off - do nothing
                if !PROGRAM_DATA.get_status() {
                    break 'keydown;
                }

                // Redraw popup on shift/caps lock changes
                let is_repeat = GetKeyState(msg_vk.0.into()) & 0x8000u16 as i16 != 0;
                if control
                    && (msg_vk == VK_CAPITAL || msg_vk == VK_LSHIFT || msg_vk == VK_RSHIFT)
                    && !is_repeat
                {
                    popup::update_popup();
                }

                // ignore Caps Lock, L an R Shift
                if msg_vk == VK_PACKET
                    || msg_vk == VK_BACK
                    || msg_vk == VK_CAPITAL
                    || msg_vk == VK_LSHIFT
                    || msg_vk == VK_RSHIFT
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
                popup::update_popup();

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
                        popup::update_popup();
                    }
                    // Redraw popup if shift was released and CONTROL_KEY is pressed
                    VK_LSHIFT | VK_RSHIFT => {
                        let control = GetKeyState(CONTROL_KEY.0.into()) & 0x8000u16 as i16 != 0;
                        if control {
                            popup::update_popup();
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
