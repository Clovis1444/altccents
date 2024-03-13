// accent.rs

use crate::data::{self, *};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

struct InputState {
    previous_accent: Option<AccentKey>,
    press_count: usize,
    capture_wm_char: bool,
}

static mut INPUT_STATE: InputState = InputState {
    previous_accent: None,
    press_count: 0,
    capture_wm_char: false,
};

pub fn get_wm_char_capturer_state() -> bool {
    unsafe { INPUT_STATE.capture_wm_char }
}

pub fn enable_wm_char_capturer() {
    unsafe {
        INPUT_STATE.capture_wm_char = true;
    }
}

pub fn disable_wm_char_capturer() {
    unsafe {
        INPUT_STATE.capture_wm_char = false;
    }
}

pub fn get_input_state() -> Option<(AccentKey, usize)> {
    unsafe {
        match INPUT_STATE.previous_accent {
            Some(val) => Some((val, INPUT_STATE.press_count)),
            None => None,
        }
    }
}

// TODO: Implement reseting state when window was changed
pub fn update_input_state(current_key: &VIRTUAL_KEY) {
    let current_accent = match data::AccentKey::from_vk(current_key) {
        Some(val) => val,
        None => unsafe {
            INPUT_STATE.previous_accent = None;
            INPUT_STATE.press_count = 0;
            return;
        },
    };
    unsafe {
        let max = accent_amount(&current_accent)
            .expect("Accent must exists due to the previous check")
            - 1;

        if INPUT_STATE.previous_accent != None
            && INPUT_STATE.previous_accent.unwrap() == current_accent
            && INPUT_STATE.press_count < max
        {
            INPUT_STATE.press_count += 1;
        } else {
            INPUT_STATE.previous_accent = Some(current_accent);
            INPUT_STATE.press_count = 0;
        }
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
