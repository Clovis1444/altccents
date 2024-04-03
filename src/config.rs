// config.rs

use windows::{
    core::{w, PCWSTR},
    Win32::{
        Foundation::COLORREF,
        UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK},
    },
};

pub const CONTROL_KEY: VIRTUAL_KEY = VK_NUMLOCK;
pub const USE_TIMER: bool = false;
pub const USE_SOUND: bool = true;
// In milliseconds
pub const MAX_KEY_INTERVAL: u32 = 1000;
pub const DEFAULT_PROGRAM_STATUS: bool = true;

pub const PROGRAM_NAME: PCWSTR = w!("Altccents");
pub const PROGRAM_SITE: PCWSTR = w!("https://github.com/Clovis1444/altccents");
// Name of the icon in resources.rc
pub const PROGRAM_ICON_IMG: PCWSTR = w!("PROGRAM_ICON");
// Font name; not .ttf file name
pub const POPUP_FONT: PCWSTR = w!("Inter");
pub const POPUP_FONT_SIZE: i32 = 28;
pub const POPUP_FONT_COLOR: COLORREF = COLORREF { 0: 0x00E0E7E9 }; // Platinum
pub const POPUP_WINDOW_TRANSPARENT_COLOR: COLORREF = COLORREF { 0: 0x00000000 }; // black
pub const POPUP_WINDOW_TRANSPARENCY: u8 = 255;
// Should be even, do not make too small
pub const POPUP_CELL_SIZE: i32 = 55;
pub const POPUP_SELECT_CELL_SIZE: i32 = (POPUP_CELL_SIZE as f32 * 0.9) as i32;
pub const POPUP_CELL_COLOR: COLORREF = COLORREF { 0: 0x002B2117 };
pub const POPUP_SELECT_CELL_COLOR: COLORREF = COLORREF { 0: 0x0078522B };
// Use value of POPUP_CELL_SIZE for max rounding
pub const POPUP_CELL_ROUND: i32 = POPUP_CELL_SIZE / 7;
pub const POPUP_SELECT_CELL_ROUND: i32 = POPUP_SELECT_CELL_SIZE / 7;
pub const POPUP_CIRCLE_SELECTION: bool = false;
pub const TIMER_ID: usize = 1337;
pub const TRAY_ICON_ID: u32 = 1337;
// Name of the icon in resources.rc
pub const TRAY_ICON_IMG_ON: PCWSTR = w!("PROGRAM_ON_ICON");
// Name of the icon in resources.rc
pub const TRAY_ICON_IMG_OFF: PCWSTR = w!("PROGRAM_OFF_ICON");
pub const TRAY_ICON_TIP_TEXT: &str = "Altccents";
// Should be in range 32768..49151
pub const TRAY_CALLBACK_MESSAGE: u32 = 33333;
pub const SWITCH_PROGRAM_STATE_BUTTON_ID: u32 = 100;
pub const QUIT_BUTTON_ID: u32 = 101;
pub const ABOUT_BUTTON_ID: u32 = 102;
