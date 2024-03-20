// session.rs

use super::config::*;

use windows::Win32::{
    Foundation::HWND,
    UI::{Shell::NOTIFYICONDATAW, WindowsAndMessaging::HHOOK},
};

pub struct ProgramData {
    hwnd: Option<HWND>,
    hhook: Option<HHOOK>,
    tray_icon_data: Option<NOTIFYICONDATAW>,
    status: bool,
}

// hwnd, hhook, tray_icon_data should be set at program start up
pub static mut PROGRAM_DATA: ProgramData = ProgramData {
    hwnd: None,
    hhook: None,
    tray_icon_data: None,
    status: DEFAULT_PROGRAM_STATUS,
};

impl ProgramData {
    pub fn set_hwnd(&mut self, hwnd: HWND) {
        self.hwnd = Some(hwnd)
    }
    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
            .expect("PROGRAM_DATA.hwnd should be set at program start up")
    }

    pub fn set_hhook(&mut self, hhook: HHOOK) {
        self.hhook = Some(hhook)
    }
    pub fn get_hhook(&self) -> HHOOK {
        self.hhook
            .expect("PROGRAM_DATA.hhook should be set at program start up")
    }

    pub fn set_tray_icon_data(&mut self, icon_data: NOTIFYICONDATAW) {
        self.tray_icon_data = Some(icon_data)
    }
    pub fn get_tray_icon_data(&self) -> NOTIFYICONDATAW {
        self.tray_icon_data
            .expect("PROGRAM_DATA.tray_icon_data should be set at program start up")
    }

    pub fn change_status(&mut self) {
        if self.status {
            self.status = false
        } else {
            self.status = true
        }
    }
    pub fn get_status(&self) -> bool {
        self.status
    }
}
