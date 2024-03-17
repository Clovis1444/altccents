// timer.rs

use super::super::config::*;
use super::accent::{self, reset_input_state};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{KillTimer, SetTimer},
};

pub fn set_timer(hwnd: HWND) {
    unsafe {
        match SetTimer(hwnd, TIMER_ID, MAX_KEY_INTERVAL, Some(timer_proc)) {
            0 => panic!("Failed to set timer."),
            _ => (),
        };
    }
}

pub fn kill_timer(hwnd: HWND, timer_id: usize) {
    unsafe {
        match KillTimer(hwnd, timer_id) {
            Ok(_) => (),
            Err(_) => println!("Failed to kill timer"),
        }
    };
}

unsafe extern "system" fn timer_proc(_hwnd: HWND, _message: u32, timer_id: usize, _curr_time: u32) {
    match timer_id {
        TIMER_ID => {
            accent::send_char_and_kill_timer();
            reset_input_state();
        }
        _ => (),
    }
}
