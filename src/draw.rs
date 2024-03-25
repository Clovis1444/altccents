// draw.rs

use windows::Win32::{Foundation::RECT, Graphics::Gdi::*};

use super::config::*;

use super::hook::accent;
use super::hook::data;

pub fn draw(hdc: HDC) {
    unsafe {
        let (key, index) = match accent::get_input_state() {
            Some((key, index)) => (key, index),
            None => return,
        };
        let is_capital = accent::check_if_capital();

        let mut u16_arr: Vec<u16> = vec![];
        for i in data::get_accent_chars(key, is_capital) {
            u16_arr.push(*i as u16);
        }

        // Font
        let font: HFONT = CreateFontW(
            POPUP_FONT_SIZE,
            POPUP_FONT_SIZE,
            0,
            0,
            FW_NORMAL.0 as i32,
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

        let old_font = SelectObject(hdc, font);
        SetTextColor(hdc, POPUP_FONT_COLOR);
        SetBkMode(hdc, TRANSPARENT);

        // Brush
        let cell_brush = CreateSolidBrush(POPUP_CELL_COLOR);
        let select_cell_brush = CreateSolidBrush(POPUP_SELECT_CELL_COLOR);

        // Draw
        let len = u16_arr.len() as i32;

        let left = 0;
        let top = 0;
        let right = 50;
        let bottom = 50;

        let mut count: usize = 0;
        while count < len as usize {
            let mut text_rect = RECT {
                left: left + (count as i32) * POPUP_CELL_SIZE,
                top: top,
                right: right + (count as i32) * POPUP_CELL_SIZE,
                bottom: bottom,
            };

            let brush: HBRUSH;
            if count == index {
                brush = select_cell_brush;
            } else {
                brush = cell_brush;
            }

            FillRect(hdc, &text_rect, brush);

            DrawTextExW(
                hdc,
                &mut u16_arr[count..=count],
                &mut text_rect,
                DT_CENTER | DT_SINGLELINE | DT_VCENTER,
                None,
            );
            count += 1;
        }

        // Reset default font
        SelectObject(hdc, old_font);
        // Delete created objects
        DeleteObject(font);
        DeleteObject(cell_brush);
        DeleteObject(select_cell_brush);
    }
}
