// accent.rs

use super::super::{config::*, window::get_main_hwnd};
use super::{
    data::{self, *},
    timer::{kill_timer, set_timer},
};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

struct InputState {
    accent: Option<AccentKey>,
    press_count: usize,
}

static mut INPUT_STATE: InputState = InputState {
    accent: None,
    press_count: 0,
};

pub fn get_input_state() -> Option<(AccentKey, usize)> {
    unsafe {
        match INPUT_STATE.accent {
            Some(val) => Some((val, INPUT_STATE.press_count)),
            None => None,
        }
    }
}

// TODO: Implement reseting state when window was changed
pub fn update_input_state(current_key: &VIRTUAL_KEY) {
    unsafe {
        let current_accent = match data::AccentKey::from_vk(current_key) {
            Some(val) => val,
            None => {
                INPUT_STATE.accent = None;
                INPUT_STATE.press_count = 0;
                return;
            }
        };
        let max = accent_amount(&current_accent)
            .expect("Accent must exists due to the previous check")
            - 1;

        if INPUT_STATE.accent != None
            && INPUT_STATE.accent.unwrap() == current_accent
            && INPUT_STATE.press_count < max
        {
            INPUT_STATE.press_count += 1;
        } else {
            INPUT_STATE.accent = Some(current_accent);
            INPUT_STATE.press_count = 0;
        }

        // If current key is accent key -> set timer
        set_timer(get_main_hwnd());
    }
}

pub fn reset_input_state() {
    unsafe {
        INPUT_STATE.accent = None;
        INPUT_STATE.press_count = 0;
    }
}

pub fn send_char(ch: char) {
    let pinputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&pinputs, 40);
    }
}

// Will not send char if there is no accent in INPUT_STATE
pub fn send_char_and_kill_timer() {
    unsafe {
        match get_input_state() {
            Some((key, index)) => {
                kill_timer(get_main_hwnd(), TIMER_ID);

                let is_capital = GetKeyState(VK_CAPITAL.0.into()) & 0x0001 != 0;

                send_char(get_accent(key, is_capital, index));
            }
            None => {
                // println!("Failed to send char: not an accent key")
                ()
            }
        }
    }
}

// Does not needed when using WH_KEYBOARDHOOK_LL
#[allow(dead_code)]
pub fn send_vk_back() {
    unsafe {
        keybd_event(VK_BACK.0.try_into().unwrap(), 0, KEYEVENTF_EXTENDEDKEY, 0);
        keybd_event(VK_BACK.0.try_into().unwrap(), 0, KEYEVENTF_KEYUP, 0);
    }
}
