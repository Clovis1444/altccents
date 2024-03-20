// config.rs

use windows::{
    core::{w, PCWSTR},
    Win32::UI::{
        Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK},
        WindowsAndMessaging::{IDI_QUESTION, IDI_SHIELD},
    },
};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
// In milliseconds
pub const MAX_KEY_INTERVAL: u32 = 1000;
pub const DEFAULT_PROGRAM_STATUS: bool = true;

pub const PROGRAM_NAME: PCWSTR = w!("Altccents");
pub const TIMER_ID: usize = 1337;
pub const TRAY_ICON_ID: u32 = 1337;
pub const TRAY_ICON_IMG_ON: PCWSTR = IDI_QUESTION;
pub const TRAY_ICON_IMG_OFF: PCWSTR = IDI_SHIELD;
pub const TRAY_ICON_TIP_TEXT: &str = "Altccents";
// Should be in range 32768..49151
pub const TRAY_CALLBACK_MESSAGE: u32 = 33333;
pub const SWITCH_PROGRAM_STATE_BUTTON_ID: u32 = 100;
pub const QUIT_BUTTON_ID: u32 = 101;

// TODO: load custom icons here
