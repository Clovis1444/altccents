// config.rs

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
// In ms
pub const MAX_KEY_INTERVAL: u32 = 1000;
pub const TIMER_ID: usize = 1337;
