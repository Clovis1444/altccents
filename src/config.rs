// config.rs

use windows::{
    core::{w, PCWSTR},
    Win32::{
        Foundation::COLORREF,
        UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_NUMLOCK},
    },
};

// Settings that user can change
struct Settings {
    control_key: VIRTUAL_KEY,
    use_timer: bool,
    max_key_interval: u32,
    use_sound: bool,
    default_program_status: bool,
    popup_font_size: i32,
    popup_window_transparency: u8,
    popup_cell_size: i32,
    popup_circle_selection: bool,
}
impl Settings {
    const POPUP_CELL_SIZE: i32 = 55;
    const POPUP_SELECT_CELL_SCALE: f32 = 0.9;
    const POPUP_ROUND_FACTOR: i32 = 7;
}

// Default settings
static mut SETTINGS: Settings = Settings {
    control_key: VK_NUMLOCK,
    use_timer: false,
    // In milliseconds
    max_key_interval: 1000,
    use_sound: true,
    default_program_status: true,
    popup_font_size: 40,
    popup_window_transparency: 255,
    popup_cell_size: Settings::POPUP_CELL_SIZE,
    popup_circle_selection: false,
};

pub const PROGRAM_NAME: PCWSTR = w!("Altccents");
pub const PROGRAM_SITE: PCWSTR = w!("https://github.com/Clovis1444/altccents");
// Name of the icon in resources.rc
pub const PROGRAM_ICON_IMG: PCWSTR = w!("PROGRAM_ICON");
// Font name; not .ttf file name
pub const POPUP_FONT: PCWSTR = w!("Inter");
pub const POPUP_FONT_COLOR: COLORREF = COLORREF { 0: 0x00E0E7E9 }; // Platinum
pub const POPUP_WINDOW_TRANSPARENT_COLOR: COLORREF = COLORREF { 0: 0x00000000 }; // black
pub const POPUP_CELL_COLOR: COLORREF = COLORREF { 0: 0x002B2117 };
pub const POPUP_SELECT_CELL_COLOR: COLORREF = COLORREF { 0: 0x0078522B };
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
pub const ADD_STARTUP_BUTTON_ID: u32 = 103;
pub const REMOVE_STARTUP_BUTTON_ID: u32 = 104;

macro_rules! apply_attr {
    { #!$attr:tt $($it:item)* } => {
        $(
            #$attr
            $it
        )*
    }
}

apply_attr! {
#![allow(non_snake_case)]

pub fn CONTROL_KEY() -> VIRTUAL_KEY {
    unsafe { SETTINGS.control_key }
}
pub fn USE_TIMER() -> bool {
    unsafe { SETTINGS.use_timer }
}
pub fn MAX_KEY_INTERVAL() -> u32 {
    unsafe { SETTINGS.max_key_interval }
}
pub fn USE_SOUND() -> bool {
    unsafe { SETTINGS.use_sound }
}
pub fn DEFAULT_PROGRAM_STATUS() -> bool {
    unsafe { SETTINGS.default_program_status }
}
pub fn POPUP_FONT_SIZE() -> i32 {
    unsafe { SETTINGS.popup_font_size }
}
pub fn POPUP_WINDOW_TRANSPARENCY() -> u8 {
    unsafe { SETTINGS.popup_window_transparency }
}
pub fn POPUP_CELL_SIZE() -> i32 {
    unsafe { SETTINGS.popup_cell_size }
}
pub fn POPUP_CIRCLE_SELECTION() -> bool {
    unsafe { SETTINGS.popup_circle_selection }
}
pub fn POPUP_SELECT_CELL_SIZE() -> i32{
    unsafe{
        (SETTINGS.popup_cell_size as f32 * Settings::POPUP_SELECT_CELL_SCALE) as i32
    }
}
// Set POPUP_ROUND_FACTOR to "1" for max rounding
pub fn POPUP_CELL_ROUND() -> i32 {
    unsafe {SETTINGS.popup_cell_size / Settings::POPUP_ROUND_FACTOR}
}
pub fn POPUP_SELECT_CELL_ROUND() -> i32{
        POPUP_SELECT_CELL_SIZE() / Settings::POPUP_ROUND_FACTOR
}
}
