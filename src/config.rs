// config.rs

use windows::{
    core::{w, PCWSTR},
    Win32::{
        Foundation::COLORREF,
        UI::{
            Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK},
            WindowsAndMessaging::{IDI_QUESTION, IDI_SHIELD},
        },
    },
};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
pub const USE_TIMER: bool = false;
// In milliseconds
pub const MAX_KEY_INTERVAL: u32 = 1000;
pub const DEFAULT_PROGRAM_STATUS: bool = true;

pub const PROGRAM_NAME: PCWSTR = w!("Altccents");
pub const PROGRAM_SITE: PCWSTR = w!("https://github.com/Clovis1444/altccents");
pub const POPUP_FONT: PCWSTR = w!("Georgia");
pub const POPUP_FONT_SIZE: i32 = 20;
pub const POPUP_FONT_COLOR: COLORREF = COLORREF { 0: 0x0000FF00 }; // green
pub const POPUP_WINDOW_TRANSPARENT_COLOR: COLORREF = COLORREF { 0: 0x00000000 }; // black
pub const POPUP_WINDOW_TRANSPARENCY: u8 = 255;
pub const POPUP_CELL_SIZE: i32 = 50;
pub const POPUP_CELL_COLOR: COLORREF = COLORREF { 0: 0x00FF0000 }; // blue
pub const POPUP_SELECT_CELL_COLOR: COLORREF = COLORREF { 0: 0x00FFFFFF }; // white
pub const TIMER_ID: usize = 1337;
pub const TRAY_ICON_ID: u32 = 1337;
pub const TRAY_ICON_IMG_ON: PCWSTR = IDI_QUESTION;
pub const TRAY_ICON_IMG_OFF: PCWSTR = IDI_SHIELD;
pub const TRAY_ICON_TIP_TEXT: &str = "Altccents";
// Should be in range 32768..49151
pub const TRAY_CALLBACK_MESSAGE: u32 = 33333;
pub const SWITCH_PROGRAM_STATE_BUTTON_ID: u32 = 100;
pub const QUIT_BUTTON_ID: u32 = 101;
pub const ABOUT_BUTTON_ID: u32 = 102;

// TODO: load custom icons here
