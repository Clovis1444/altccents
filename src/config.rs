// config.rs

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
// In milliseconds
pub const MAX_KEY_INTERVAL: u32 = 1000;

pub const TIMER_ID: usize = 1337;
pub const TRAY_ICON_ID: u32 = 1337;
// Should be in range 32768..49151
pub const TRAY_CALLBACK_MESSAGE: u32 = 33333;
