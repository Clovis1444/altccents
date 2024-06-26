//! # resources
//! `resources.rs` module is responsible for loading and unloading resources from `/resources`.

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HANDLE,
        Graphics::Gdi::*,
        System::LibraryLoader::{
            FindResourceW, GetModuleHandleW, LoadResource, LockResource, SizeofResource,
        },
        UI::WindowsAndMessaging::RT_FONT,
    },
};

struct ResourceData {
    h_font: Option<HANDLE>,
}

/// Object that stores all loaded resource handles.
static mut RESOURCE_DATA: ResourceData = ResourceData { h_font: None };

/// Loads resources so we can us them in our program.
pub fn init_resources() {
    unsafe {
        // Return if resources was already initialised
        if let Some(_) = RESOURCE_DATA.h_font {
            return;
        }

        let instance = GetModuleHandleW(None).unwrap();
        // Load font resource
        // lpname - name of the font in resources.rc
        let res = FindResourceW(instance, PCWSTR(1 as *const u16), RT_FONT);
        let font_mem = LoadResource(instance, res).unwrap();
        let font_data = LockResource(font_mem);

        let res_size = SizeofResource(instance, res);
        let n_fonts: u32 = 0;
        let hfont = AddFontMemResourceEx(font_data, res_size, None, &n_fonts);

        RESOURCE_DATA.h_font = Some(hfont);
    }
}

/// Unloads resources when we don't need them anymore.
pub fn unload_resources() {
    unsafe {
        match RESOURCE_DATA.h_font {
            None => (),
            Some(handle) => {
                RemoveFontMemResourceEx(handle);
                RESOURCE_DATA.h_font = None;
            }
        }
    }
}
