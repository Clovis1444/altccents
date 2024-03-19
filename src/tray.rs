// tray.rs

use super::config::*;
use std::mem::size_of;
use windows::{
    core::{h, PWSTR},
    Win32::{
        Foundation::{HINSTANCE, HWND, POINT},
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

pub fn get_tray_icon_data(hwnd: HWND) -> NOTIFYICONDATAW {
    unsafe {
        let mut tip_text: [u16; 128] = [0; 128];
        {
            let tip = "Altccents".to_string();
            assert!(tip.len() <= 127, "Tip text can have a maximum of 128 characters, including the terminating null character");
            let tip: Vec<u16> = tip.chars().map(|c| c as u16).collect();
            let mut index: usize = 0;
            for i in tip {
                tip_text[index] = i;
                index += 1;
            }
        }

        let icon_data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: TRAY_CALLBACK_MESSAGE,
            hIcon: LoadIconW(HINSTANCE { 0: 0 }, IDI_QUESTION).unwrap(),
            szTip: tip_text,
            ..Default::default()
        };
        icon_data
    }
}

// TODO: implement change_tray_icon()
pub fn add_tray_icon(icon_data: &NOTIFYICONDATAW) {
    unsafe {
        match Shell_NotifyIconW(NIM_ADD, icon_data).as_bool() {
            false => panic!("Failed to add tray icon"),
            true => (),
        };
    }
}

pub fn delete_tray_icon(icon_data: &NOTIFYICONDATAW) {
    unsafe {
        match Shell_NotifyIconW(NIM_DELETE, icon_data).as_bool() {
            false => panic!("Failed to delete tray icon"),
            true => (),
        };
    }
}

// TODO: implement DestroyMenu()
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu
pub fn context_menu(hwnd: HWND) {
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

        let button1_text: PWSTR;
        let button2_text: PWSTR;
        {
            if get_program_status() {
                button1_text = PWSTR::from_raw(h!("Altccents is ON").as_ptr() as *mut u16);
                button2_text = PWSTR::from_raw(h!("Turn off altccents").as_ptr() as *mut u16);
            } else {
                button1_text = PWSTR::from_raw(h!("Altccents is OFF").as_ptr() as *mut u16);
                button2_text = PWSTR::from_raw(h!("Turn on altccents").as_ptr() as *mut u16);
            }
        }

        // Program status text
        let button1 = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_STATE,
            fState: MFS_GRAYED | MFS_DEFAULT,
            dwTypeData: button1_text,
            ..Default::default()
        };
        match InsertMenuItemW(menu, 1, true, &button1) {
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
        let button2 = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: SWITCH_PROGRAM_STATE_BUTTON_ID,
            dwTypeData: button2_text,
            ..Default::default()
        };
        match InsertMenuItemW(menu, 3, true, &button2) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        // Quit button
        let button3 = MENUITEMINFOW {
            cbSize: size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_STRING | MIIM_ID,
            wID: QUIT_BUTTON_ID,
            dwTypeData: PWSTR::from_raw(h!("Quit").as_ptr() as *mut u16),
            ..Default::default()
        };
        match InsertMenuItemW(menu, 4, true, &button3) {
            Ok(_) => (),
            Err(_) => panic!("Failed to insert menu item"),
        };

        TrackPopupMenu(
            menu,
            TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_LEFTBUTTON,
            cursor_pos.x,
            cursor_pos.y,
            0,
            hwnd,
            None,
        );
    }
}
