// hook.rs

use windows::core::*;
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    UI::WindowsAndMessaging::*,
};

// Install hook
pub fn setup_hook() -> HHOOK {
    unsafe {
        let hmode = LoadLibraryW(w!(
            "E:\\Projects\\altccents\\target\\debug\\altccents_dll.dll"
        ));

        let callback_name = PCSTR::from_raw("wh_callback\0".as_ptr());
        let wh_callback_address = GetProcAddress(hmode.clone().unwrap(), *&callback_name).unwrap();

        let wh_callback: unsafe extern "system" fn(
            code: i32,
            w_param: WPARAM,
            l_param: LPARAM,
        ) -> LRESULT = std::mem::transmute(wh_callback_address);

        let hook = SetWindowsHookExW(WH_GETMESSAGE, Some(wh_callback), hmode.unwrap(), 0);

        match hook {
            Err(err) => panic!("Failed to install hook: {err}!"),
            Ok(hhk) => {
                println!("Hook was installed successefully.");
                hhk
            }
        }
    }
}

// Uninstall hook
pub fn remove_hook(hook: HHOOK) {
    unsafe {
        let result = UnhookWindowsHookEx(hook);

        match result {
            Err(err) => panic!("Failed to remove hook: {err}!"),
            Ok(_) => println!("Hook was removed successefully."),
        }
    }
}
