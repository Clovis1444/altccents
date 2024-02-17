// hook.rs

use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

// Install hook
pub fn setup_hook() -> HHOOK {
    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(callback), None, 0);

        match hook {
            Err(_) => panic!("Failed to install hook!"),
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
            Err(_) => panic!("Failed to remove hook!"),
            Ok(_) => println!("Hook was removed successefully."),
        }
    }
}

// Main hook logic
unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION.try_into().unwrap() {
        match w_param.0 as u32 {
            WM_KEYDOWN => {
                let key_info: *const KBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                println!("Key \'{}\' was pressed.", (*key_info).vkCode);
                // Uncomment the following line to cath the keyboard event
                // return LRESULT(1);
            }
            _ => (),
        }
    }
    // Call next hook from some other application
    CallNextHookEx(None, code, w_param, l_param)
}
