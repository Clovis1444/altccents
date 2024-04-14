//! # config
//! `config.rs` contains all public constants, runtime changable settings and API to interact with them.

mod args;
use std::path::Path;

pub use args::get_args;

use mslnk::ShellLink;
use windows::{
    core::{w, PCWSTR},
    Win32::{
        Foundation::COLORREF,
        UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_F12, VK_NUMLOCK, VK_SCROLL},
    },
};

/// Struct containing all the settings that can be changed at runtime.
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
    popup_select_cell_scale: f32,
    popup_round_factor: i32,
}
impl Settings {
    const VALID_CONTROL_KEYS: [VIRTUAL_KEY; 3] = [VK_NUMLOCK, VK_SCROLL, VK_F12];
}

// Default settings
impl Default for Settings {
    fn default() -> Settings {
        Settings {
            control_key: VK_NUMLOCK,
            use_timer: false,
            // In milliseconds
            max_key_interval: 1000,
            use_sound: true,
            default_program_status: true,
            popup_font_size: 50,
            popup_window_transparency: 255,
            popup_cell_size: 70,
            popup_circle_selection: false,
            popup_select_cell_scale: 0.9,
            // Set popup_round_factor to "1" for max rounding
            popup_round_factor: 7,
        }
    }
}

/// Object that stores all setting values.
static mut SETTINGS: Settings = Settings {
    control_key: VK_NUMLOCK,
    use_timer: false,
    // In milliseconds
    max_key_interval: 1000,
    use_sound: true,
    default_program_status: true,
    popup_font_size: 50,
    popup_window_transparency: 255,
    popup_cell_size: 70,
    popup_circle_selection: false,
    popup_select_cell_scale: 0.9,
    // Set popup_round_factor to "1" for max rounding
    popup_round_factor: 7,
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
pub const SET_SETTINGS_BUTTON_ID: u32 = 105;
pub const RESET_SETTINGS_BUTTON_ID: u32 = 106;

/// Initialize `SETTINGS` depending on startup CLI arguments.
/// > Note: this function must be called before `window::create_window()`.
pub fn init_settings() {
    let mut args = std::env::args();
    // Skip binary path arg
    args.next();

    let mut options = String::new();
    for i in args {
        args::validate_arg(&i);
        options.push_str(&i);
        options.push(' ');
    }
}

/// Use this function to change `SETTINGS` at runtime.
pub fn change_settings(options: Vec<&str>) {
    let mut opts = String::new();
    for i in options {
        args::validate_arg(i);
        opts.push_str(i);
        opts.push(' ');
    }

    // Add startup options to shortcut if it exists
    unsafe {
        let lnk = Path::new(&std::env::var("APPDATA").unwrap())
            .join("Microsoft/Windows/Start Menu/Programs/Startup")
            .join(PROGRAM_NAME.to_string().unwrap() + ".lnk");

        if lnk.exists() {
            let target = std::env::current_exe().unwrap();

            let mut sl = ShellLink::new(target).unwrap();
            sl.set_arguments(Some(get_args()));
            sl.create_lnk(lnk).unwrap();
        }
    }
    // TODO: create pub const for link path?
}

/// Reset `SETTINGS` to default values.
pub fn reset_settings() {
    unsafe {
        SETTINGS = Settings::default();

        // Update transparancy
        use super::session::PROGRAM_DATA;
        use windows::Win32::UI::WindowsAndMessaging::{
            SetLayeredWindowAttributes, LWA_ALPHA, LWA_COLORKEY,
        };
        let _ = SetLayeredWindowAttributes(
            PROGRAM_DATA.get_hwnd(),
            POPUP_WINDOW_TRANSPARENT_COLOR,
            POPUP_WINDOW_TRANSPARENCY(),
            LWA_ALPHA | LWA_COLORKEY,
        );

        // Add startup options to shortcut if it exists
        let lnk = Path::new(&std::env::var("APPDATA").unwrap())
            .join("Microsoft/Windows/Start Menu/Programs/Startup")
            .join(PROGRAM_NAME.to_string().unwrap() + ".lnk");

        if lnk.exists() {
            let target = std::env::current_exe().unwrap();

            let mut sl = ShellLink::new(target).unwrap();
            sl.set_arguments(None);
            sl.create_lnk(lnk).unwrap();
        }
    }
}

macro_rules! apply_attr {
    { #!$attr:tt $($it:item)* } => {
        $(
            /// `SETTINGS` getter.
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
        (SETTINGS.popup_cell_size as f32 * SETTINGS.popup_select_cell_scale) as i32
    }
}
pub fn POPUP_CELL_ROUND() -> i32 {
    unsafe {SETTINGS.popup_cell_size / SETTINGS.popup_round_factor}
}
pub fn POPUP_SELECT_CELL_ROUND() -> i32{
       unsafe{ POPUP_SELECT_CELL_SIZE() / SETTINGS.popup_round_factor}
}

}
