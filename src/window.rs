// window.rs

use mslnk::ShellLink;
use std::path::Path;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::LibraryLoader::*,
        UI::{Shell::ShellExecuteW, WindowsAndMessaging::*},
    },
};

use super::{config::*, popup, session::PROGRAM_DATA, tray};

pub fn create_window() -> Result<HWND> {
    unsafe {
        let hwnd: HWND;
        // Module handle. In this case - our binary handle
        let instance = GetModuleHandleW(None)?;
        debug_assert!(instance.0 != 0);

        // Window attributes
        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: PROGRAM_NAME,
            hIcon: LoadIconW(GetModuleHandleW(None).unwrap(), PROGRAM_ICON_IMG).unwrap(),
            hbrBackground: HBRUSH {
                0: GetStockObject(BLACK_BRUSH).0,
            },

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        // Register class to use it in "CreateWindowExW"
        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        // Create window
        hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_LAYERED,
            PROGRAM_NAME,
            PROGRAM_NAME,
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            0,
            0,
            None,
            None,
            instance,
            None,
        );

        let _ = SetLayeredWindowAttributes(
            hwnd,
            POPUP_WINDOW_TRANSPARENT_COLOR,
            POPUP_WINDOW_TRANSPARENCY,
            LWA_ALPHA | LWA_COLORKEY,
        );

        Ok(hwnd)
    }
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
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            TRAY_CALLBACK_MESSAGE => match l_param.0 as u32 {
                WM_LBUTTONDOWN => {
                    PROGRAM_DATA.change_status(false);
                    tray::update_tray_icon(&mut PROGRAM_DATA);
                    LRESULT(0)
                }
                WM_RBUTTONDOWN => {
                    tray::context_menu(&PROGRAM_DATA);
                    LRESULT(0)
                }
                _ => DefWindowProcW(window, message, w_param, l_param),
            },
            WM_COMMAND => match w_param.0 as u32 {
                SWITCH_PROGRAM_STATE_BUTTON_ID => {
                    PROGRAM_DATA.change_status(false);
                    tray::update_tray_icon(&mut PROGRAM_DATA);
                    LRESULT(0)
                }
                ABOUT_BUTTON_ID => {
                    ShellExecuteW(
                        HWND::default(),
                        PCWSTR::null(),
                        PROGRAM_SITE,
                        PCWSTR::null(),
                        PCWSTR::null(),
                        SW_SHOW,
                    );
                    LRESULT(0)
                }
                QUIT_BUTTON_ID => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                ADD_STARTUP_BUTTON_ID => {
                    let target = std::env::current_exe().unwrap();
                    let lnk = Path::new(&std::env::var("APPDATA").unwrap())
                        .join("Microsoft/Windows/Start Menu/Programs/Startup")
                        .join(PROGRAM_NAME.to_string().unwrap() + ".lnk");

                    let sl = ShellLink::new(target).unwrap();
                    sl.create_lnk(lnk).unwrap();

                    LRESULT(0)
                }
                REMOVE_STARTUP_BUTTON_ID => {
                    let lnk = Path::new(&std::env::var("APPDATA").unwrap())
                        .join("Microsoft/Windows/Start Menu/Programs/Startup")
                        .join(PROGRAM_NAME.to_string().unwrap() + ".lnk");

                    std::fs::remove_file(lnk).unwrap();

                    LRESULT(0)
                }
                _ => DefWindowProcW(window, message, w_param, l_param),
            },
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(PROGRAM_DATA.get_hwnd(), &mut ps);
                //

                popup::draw(hdc);

                //
                EndPaint(PROGRAM_DATA.get_hwnd(), &mut ps);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, w_param, l_param),
        }
    }
}
