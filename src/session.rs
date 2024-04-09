// session.rs

use super::config::*;

use windows::{
    core::w,
    Win32::{
        Foundation::HWND,
        Media::Audio::{PlaySoundW, SND_ASYNC},
        System::LibraryLoader::GetModuleHandleW,
        UI::{Shell::NOTIFYICONDATAW, WindowsAndMessaging::HHOOK},
    },
};

pub struct ProgramData {
    hwnd: Option<HWND>,
    hhook: Option<HHOOK>,
    tray_icon_data: Option<NOTIFYICONDATAW>,
    status: bool,
    settings_options: Option<String>,
}

// hwnd, hhook, tray_icon_data should be set at program start up
pub static mut PROGRAM_DATA: ProgramData = ProgramData {
    hwnd: None,
    hhook: None,
    tray_icon_data: None,
    status: false,
    settings_options: None,
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

    pub fn set_status(&mut self, status: bool) {
        self.status = status;
    }
    pub fn change_status(&mut self, play_sound: bool) {
        unsafe {
            if self.status {
                if play_sound && USE_SOUND() {
                    PlaySoundW(w!("SystemHand"), GetModuleHandleW(None).unwrap(), SND_ASYNC);
                }
                self.status = false
            } else {
                if play_sound && USE_SOUND() {
                    PlaySoundW(
                        w!("SystemQuestion"),
                        GetModuleHandleW(None).unwrap(),
                        SND_ASYNC,
                    );
                }
                self.status = true
            }
        }
    }
    pub fn get_status(&self) -> bool {
        self.status
    }

    pub fn get_settings_options(&self) -> Option<String> {
        self.settings_options.clone()
    }
    pub fn set_settings_options(&mut self, options: Option<String>) {
        match options {
            Some(x) if x.is_empty() => self.settings_options = None,
            _ => self.settings_options = options,
        }
    }
}
