//! # tray
//! `tray.rs` module contains everything related to the tray icon.
//!
//! # Add new context menu button
//! To add a new button or change an existing one see `context_menu()`. The callback logic is defined in `window.rs`.

use super::{
    config::*,
    session::{self, PROGRAM_DATA},
};
use std::{mem::size_of, path::Path};
use windows::{
    core::{h, PCWSTR, PWSTR},
    Win32::{
        Foundation::{LPARAM, POINT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

/// Returns initial tray icon data and pass it to `session::PROGRAM_DATA`.
pub fn init_tray_icon_data(program_data: &session::ProgramData) -> NOTIFYICONDATAW {
    unsafe {
        let mut tip_text: [u16; 128] = [0; 128];
        {
            let tip = TRAY_ICON_TIP_TEXT.to_string();
            assert!(tip.len() <= 127, "Tip text can have a maximum of 128 characters, including the terminating null character");
            let tip: Vec<u16> = tip.chars().map(|c| c as u16).collect();
            let mut index: usize = 0;
            for i in tip {
                tip_text[index] = i;
                index += 1;
            }
        }

        let icon_img: PCWSTR;
        {
            if program_data.get_status() {
                icon_img = TRAY_ICON_IMG_ON;
            } else {
                icon_img = TRAY_ICON_IMG_OFF;
            }
        }

        let icon_data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: program_data.get_hwnd(),
            uID: TRAY_ICON_ID,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: TRAY_CALLBACK_MESSAGE,
            hIcon: LoadIconW(GetModuleHandleW(None).unwrap(), icon_img).unwrap(),
            szTip: tip_text,
            ..Default::default()
        };
        icon_data
    }
}

/// Adds Altccents icon to tray.
pub fn add_tray_icon(program_data: &session::ProgramData) {
    unsafe {
        match Shell_NotifyIconW(NIM_ADD, &program_data.get_tray_icon_data()).as_bool() {
            false => panic!("Failed to add tray icon"),
            true => (),
        };
    }
}

/// Removes Altccents icon from tray.
pub fn delete_tray_icon(program_data: &session::ProgramData) {
    unsafe {
        match Shell_NotifyIconW(NIM_DELETE, &program_data.get_tray_icon_data()).as_bool() {
            false => panic!("Failed to delete tray icon"),
            true => (),
        };
    }
}

/// Updates tray icon picture depending on the current program state.
pub fn update_tray_icon(program_data: &mut session::ProgramData) {
    unsafe {
        let new_icon: PCWSTR;
        {
            if program_data.get_status() {
                new_icon = TRAY_ICON_IMG_ON;
            } else {
                new_icon = TRAY_ICON_IMG_OFF;
            }
        }

        let mut icon_data = program_data.get_tray_icon_data();
        icon_data.hIcon = LoadIconW(GetModuleHandleW(None).unwrap(), new_icon).unwrap();
        program_data.set_tray_icon_data(icon_data);

        match Shell_NotifyIconW(NIM_MODIFY, &icon_data).as_bool() {
            false => panic!("Failed to modify tray icon"),
            true => (),
        };
    }
}

/// Render tray context menu.
/// > Note: all context menu callback logic is defined in `window.rs`.
pub fn context_menu(program_data: &session::ProgramData) {
    unsafe {
        let mut cursor_pos: POINT = POINT::default();
        match GetCursorPos(&mut cursor_pos) {
            Ok(_) => (),
            Err(_) => panic!("Failed to get cursor position"),
        };

        let menu = match CreatePopupMenu() {
            Ok(val) => val,
            Err(_) => panic!("Failed to create popup menu"),
        };

        let status_text: PWSTR;
        let switch_button_text: PWSTR;
        {
            if PROGRAM_DATA.get_status() {
                status_text = PWSTR::from_raw(h!("Altccents is ON").as_ptr() as *mut u16);
                switch_button_text = PWSTR::from_raw(h!("Turn off altccents").as_ptr() as *mut u16);
            } else {
                status_text = PWSTR::from_raw(h!("Altccents is OFF").as_ptr() as *mut u16);
                switch_button_text = PWSTR::from_raw(h!("Turn on altccents").as_ptr() as *mut u16);
            }
        }
        let startup_button_text: PWSTR;
        let startup_button_id: u32;
        {
            let appdata = std::env::var("APPDATA").expect("Env variable %APPDATA% must exist");
            // Path to Altccents link in startup folder
            let link_path = Path::new(&appdata)
                .join("Microsoft/Windows/Start Menu/Programs/Startup")
                .join(PROGRAM_NAME.to_string().unwrap() + ".lnk");

            if link_path.exists() {
                startup_button_text =
                    PWSTR::from_raw(h!("Remove from startup").as_ptr() as *mut u16);
                startup_button_id = REMOVE_STARTUP_BUTTON_ID;
            } else {
                startup_button_text = PWSTR::from_raw(h!("Add to startup").as_ptr() as *mut u16);
                startup_button_id = ADD_STARTUP_BUTTON_ID;
            };
        }

        // Program status text
        let status = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_STATE,
            fState: MFS_GRAYED | MFS_DEFAULT,
            dwTypeData: status_text,
            ..Default::default()
        };
        match InsertMenuItemW(menu, 1, true, &status) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Menu separator
        let separator = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fType: MFT_SEPARATOR,
            ..Default::default()
        };

        match InsertMenuItemW(menu, 2, true, &separator) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Turn on/off button
        let switch_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: SWITCH_PROGRAM_STATE_BUTTON_ID,
            dwTypeData: switch_button_text,
            ..Default::default()
        };
        match InsertMenuItemW(menu, 3, true, &switch_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Startup button
        let startup_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: startup_button_id,
            dwTypeData: startup_button_text,
            ..Default::default()
        };
        match InsertMenuItemW(menu, 4, true, &startup_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Settings button
        let settings_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: SET_SETTINGS_BUTTON_ID,
            dwTypeData: PWSTR::from_raw(h!("Set settings from clipboard").as_ptr() as *mut u16),
            ..Default::default()
        };
        match InsertMenuItemW(menu, 5, true, &settings_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Reset settings button
        let reset_settings_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: RESET_SETTINGS_BUTTON_ID,
            dwTypeData: PWSTR::from_raw(h!("Reset settings to default").as_ptr() as *mut u16),
            ..Default::default()
        };
        match InsertMenuItemW(menu, 6, true, &reset_settings_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // About button
        let about_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: ABOUT_BUTTON_ID,
            dwTypeData: PWSTR::from_raw(h!("About").as_ptr() as *mut u16),
            ..Default::default()
        };
        match InsertMenuItemW(menu, 7, true, &about_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Quit button
        let quit_button = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: QUIT_BUTTON_ID,
            dwTypeData: PWSTR::from_raw(h!("Quit").as_ptr() as *mut u16),
            ..Default::default()
        };
        match InsertMenuItemW(menu, 8, true, &quit_button) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // This line fixes bug when the menu does not close until you press one of menu buttons
        // For more info see:
        // https://forums.codeguru.com/showthread.php?210985-Popup-Menu-on-system-tray-icon
        SetForegroundWindow(program_data.get_hwnd());

        TrackPopupMenu(
            menu,
            TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_LEFTBUTTON,
            cursor_pos.x,
            cursor_pos.y,
            0,
            program_data.get_hwnd(),
            None,
        );

        // This line fixes bug. See comment above
        let _ = PostMessageW(
            program_data.get_hwnd(),
            WM_NULL,
            WPARAM { 0: 0 },
            LPARAM { 0: 0 },
        );
    }
}
