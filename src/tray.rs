// tray.rs

use super::config::*;
use std::mem::size_of;
use windows::Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::{
        Shell::*,
        WindowsAndMessaging::{LoadIconW, IDI_QUESTION},
    },
};

pub fn get_tray_icon_data(hwnd: HWND) -> NOTIFYICONDATAW {
    unsafe {
        let mut tip_text: [u16; 128] = [0; 128];
        {
            let tip = "My text".to_string();
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
            // dwState: todo!(),
            // dwStateMask: todo!(),
            // szInfo: todo!(),
            // Anonymous: todo!(),
            // szInfoTitle: todo!(),
            // dwInfoFlags: todo!(),
            // guidItem: todo!(),
            // hBalloonIcon: todo!(),
            ..Default::default()
        };
        icon_data
    }
}

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
