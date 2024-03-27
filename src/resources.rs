// resources.rs

use windows::{
    core::w,
    Win32::{
        Graphics::Gdi::{CreateFontIndirectW, RemoveFontMemResourceEx, LOGFONTW},
        System::LibraryLoader::{FindResourceW, GetModuleHandleW, LoadResource, LockResource},
        UI::WindowsAndMessaging::RT_FONT,
    },
};

struct ResourceData {}

static mut resource_data: ResourceData = ResourceData {};

pub fn init_resources() {
    unsafe {
        let instance = GetModuleHandleW(None).unwrap();
        let res = FindResourceW(instance, w!("1"), RT_FONT);
        let font_mem = LoadResource(instance, res).unwrap();
        let font_data = LockResource(font_mem);

        // let logfontw = LOGFONTW {
        //     lfHeight: todo!(),
        //     lfWidth: todo!(),
        //     lfEscapement: todo!(),
        //     lfOrientation: todo!(),
        //     lfWeight: todo!(),
        //     lfItalic: todo!(),
        //     lfUnderline: todo!(),
        //     lfStrikeOut: todo!(),
        //     lfCharSet: todo!(),
        //     lfOutPrecision: todo!(),
        //     lfClipPrecision: todo!(),
        //     lfQuality: todo!(),
        //     lfPitchAndFamily: todo!(),
        //     lfFaceName: todo!(),
        // };
        // let font = CreateFontIndirectW(&logfontw);

        // Clean up font at the end
        // RemoveFontMemResourceEx(font);
    }
}
