// draw.rs
// https://www.youtube.com/watch?v=JRHltI71QiQ

use windows::Win32::{Foundation::RECT, Graphics::Gdi::*};

use super::config::*;

use super::hook::accent;
use super::hook::data;

pub fn draw(hdc: HDC) {
    unsafe {
        // let pen = CreatePen(PS_SOLID, 20, COLORREF { 0: 0x0000FF00 });
        // SelectObject(hdc, GetStockObject(BLACK_PEN));
        // SelectObject(hdc, pen);
        // SelectObject(hdc, GetStockObject(GRAY_BRUSH));
        // Rectangle(hdc, 400, 200, 100, 100);

        let key = match accent::get_input_state() {
            Some((key, _)) => key,
            None => return,
        };
        let is_capital = accent::check_if_capital();

        let mut u16_arr: Vec<u16> = vec![];
        for i in data::get_accent_chars(key, is_capital) {
            u16_arr.push(*i as u16);
        }

        let mut rect: RECT = RECT {
            left: 100,
            top: 100,
            right: 600,
            bottom: 500,
        };

        // Font
        let font: HFONT = CreateFontW(
            60,
            60,
            0,
            0,
            FW_HEAVY.0 as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET.0.into(),
            OUT_DEFAULT_PRECIS.0.into(),
            CLIP_DEFAULT_PRECIS.0.into(),
            ANTIALIASED_QUALITY.0.into(),
            (DEFAULT_PITCH.0 | FF_DONTCARE.0).into(),
            POPUP_FONT,
        );

        SelectObject(hdc, font);

        DrawTextExW(hdc, &mut u16_arr, &mut rect, DT_CENTER, None);

        DeleteObject(font);

        // DeleteObject(pen);

        // FillRect(
        //     hdc,
        //     &RECT {
        //         left: 400,
        //         top: 200,
        //         right: 100,
        //         bottom: 100,
        //     },
        //     HBRUSH {
        //         0: GetStockObject(GRAY_BRUSHs).0,
        //     },
        // );
        // draw(&mut hdc);
    }
}
