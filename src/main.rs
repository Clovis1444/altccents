// main.rs
// #![cfg_attr(not(test), windows_subsystem = "windows")]

mod config;
mod draw;
mod hook;
mod session;
mod tray;
mod window;

use session::PROGRAM_DATA;
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() -> Result<()> {
    unsafe {
        match window::create_window() {
            Err(_) => panic!("Failed to create a window!"),
            Ok(hwnd) => PROGRAM_DATA.set_hwnd(hwnd),
        };

        PROGRAM_DATA.set_tray_icon_data(tray::init_tray_icon_data(&PROGRAM_DATA));
        tray::add_tray_icon(&PROGRAM_DATA);

        PROGRAM_DATA.set_hhook(hook::setup_hook());

        // Message buffer
        let mut message = MSG::default();

        // Main message loop
        // Get messages from OS and dispatch them
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
            TranslateMessage(&message);
        }

        tray::delete_tray_icon(&PROGRAM_DATA);
        hook::remove_hook(PROGRAM_DATA.get_hhook());
        Ok(())
    }
}
