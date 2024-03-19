// config.rs

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
// In milliseconds
pub const MAX_KEY_INTERVAL: u32 = 1000;
// Set PROGRAM_STATUS to false to turn off all program functionality
static mut PROGRAM_STATUS: bool = true;

pub const TIMER_ID: usize = 1337;
pub const TRAY_ICON_ID: u32 = 1337;
// Should be in range 32768..49151
pub const TRAY_CALLBACK_MESSAGE: u32 = 33333;
pub const SWITCH_PROGRAM_STATE_BUTTON_ID: u32 = 100;
pub const QUIT_BUTTON_ID: u32 = 101;

// TODO: load custom icons here

pub fn get_program_status() -> bool {
    unsafe { PROGRAM_STATUS }
}

pub fn switch_program_status() {
    unsafe {
        if PROGRAM_STATUS == true {
            PROGRAM_STATUS = false
        } else {
            PROGRAM_STATUS = true
        }
    }
}
