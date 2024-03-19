// window.rs

use windows::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*,
    },
};

use super::{config::*, tray};

use super::config::{get_program_status, switch_program_status, TRAY_CALLBACK_MESSAGE};

static mut MAIN_WINDOW: HWND = HWND(0);

pub fn create_window() -> Result<HWND> {
    let hwnd: HWND;
    unsafe {
        // Module handle. In this case - our binary handle
        let instance = GetModuleHandleW(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = w!("my window");

        // Window attributes
        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        // Register class to use it in "CreateWindowExW"
        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        // Create window
        hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("My window title"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );
    }

    unsafe {
        if MAIN_WINDOW == HWND(0) {
            MAIN_WINDOW = hwnd
        };
    }
    Ok(hwnd)
}

// Message handler. Main window logic
extern "system" fn wndproc(
    window: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_KEYDOWN => LRESULT(0),
            WM_CHAR => LRESULT(0),
            TRAY_CALLBACK_MESSAGE => {
                match l_param.0 as u32 {
                    WM_LBUTTONDOWN => {
                        println!("Tray: Switching program status...");
                        switch_program_status();
                        println!("Tray: program status now is {}", get_program_status());
                    }
                    WM_RBUTTONDOWN => {
                        println!("Tray: right mouse click");
                        tray::context_menu(get_main_hwnd());
                    }
                    _ => (),
                }

                LRESULT(0)
            }
            WM_COMMAND => {
                match w_param.0 as u32 {
                    SWITCH_PROGRAM_STATE_BUTTON_ID => switch_program_status(),
                    QUIT_BUTTON_ID => PostQuitMessage(0),
                    _ => (),
                }
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, w_param, l_param),
        }
    }
}

pub fn get_main_hwnd() -> HWND {
    unsafe { MAIN_WINDOW }
}
