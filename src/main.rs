// main.rs

mod config;
mod hook;
mod tray;
mod window;

use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() -> Result<()> {
    unsafe {
        let hwnd = match window::create_window() {
            Err(_) => panic!("Failed to create a window!"),
            Ok(handle) => handle,
        };

        let tray_icon = tray::get_tray_icon_data(hwnd);
        tray::add_tray_icon(&tray_icon);

        let hhk = hook::setup_hook();

        // Message buffer
        let mut message = MSG::default();

        // Main message loop
        // Get messages from OS and dispatch them
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
            TranslateMessage(&message);
        }

        tray::delete_tray_icon(&tray_icon);
        hook::remove_hook(hhk);
        Ok(())
    }
}
