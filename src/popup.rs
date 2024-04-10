// popup.rs

use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::{Foundation::RECT, Graphics::Gdi::*, UI::WindowsAndMessaging::*};

use super::{
    config::*,
    hook::{accent, data},
    session::PROGRAM_DATA,
};

pub fn update_popup() {
    unsafe {
        let key = match accent::get_input_state() {
            Some((key, _)) => key,
            None => {
                ShowWindow(PROGRAM_DATA.get_hwnd(), SW_HIDE);
                return;
            }
        };
        let is_capital = accent::check_if_capital();

        let pos = get_popup_rect(data::get_accent_chars(key, is_capital).len() as i32);

        let _ = SetWindowPos(
            PROGRAM_DATA.get_hwnd(),
            HWND::default(),
            pos.left,
            pos.top,
            pos.right,
            pos.bottom,
            SWP_SHOWWINDOW | SWP_NOREDRAW | SWP_NOACTIVATE,
        );

        RedrawWindow(
            PROGRAM_DATA.get_hwnd(),
            None,
            None,
            RDW_INTERNALPAINT | RDW_INVALIDATE | RDW_ERASE,
        );
    }
}

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
            POPUP_FONT_SIZE(),
            0,
            0,
            0,
            FW_NORMAL.0 as i32,
            0,
            0,
            0,
            ANSI_CHARSET.0.into(),
            OUT_TT_PRECIS.0.into(),
            CLIP_EMBEDDED.0.into(),
            ANTIALIASED_QUALITY.0.into(),
            (FIXED_PITCH.0 | FF_ROMAN.0).into(),
            POPUP_FONT,
        );

        let old_font = SelectObject(hdc, font);
        SetTextColor(hdc, POPUP_FONT_COLOR);
        SetBkMode(hdc, TRANSPARENT);

        // Brush
        let cell_brush = CreateSolidBrush(POPUP_CELL_COLOR);
        let select_cell_brush = CreateSolidBrush(POPUP_SELECT_CELL_COLOR);

        // Pen
        let old_pen = SelectObject(hdc, GetStockObject(NULL_PEN));

        // Palette
        let palette = get_palette();
        let old_palette = SelectObject(hdc, palette);

        // Draw
        let len = u16_arr.len() as i32;

        let popup_rect = RECT {
            left: 0,
            top: 0,
            right: len * POPUP_CELL_SIZE(),
            bottom: POPUP_CELL_SIZE(),
        };

        // Cell loop
        let mut count: usize = 0;
        while count < len as usize {
            let mut text_rect = RECT {
                left: popup_rect.left + (count as i32) * POPUP_CELL_SIZE(),
                top: popup_rect.top,
                right: popup_rect.left + POPUP_CELL_SIZE() + (count as i32) * POPUP_CELL_SIZE(),
                bottom: popup_rect.bottom,
            };

            // Draw side cells
            if len == 1 || count as i32 == len - 1 || count == 0 {
                let old_brush = SelectObject(hdc, cell_brush);

                RoundRect(
                    hdc,
                    text_rect.left,
                    text_rect.top,
                    text_rect.right,
                    // TODO: fix it
                    // RoundRect() steals 1 pixel(But why...). Change value by 1 to fix it
                    text_rect.bottom + 1,
                    POPUP_CELL_ROUND(),
                    POPUP_CELL_ROUND(),
                );

                // Fill inner roundings
                if len != 1 && count == 0 {
                    let temp_rect = RECT {
                        left: text_rect.left + POPUP_CELL_SIZE() / 2,
                        top: text_rect.top,
                        right: text_rect.right,
                        bottom: text_rect.bottom,
                    };
                    FillRect(hdc, &temp_rect, cell_brush);
                }
                if len != 1 && count as i32 == len - 1 {
                    let temp_rect = RECT {
                        left: text_rect.left,
                        top: text_rect.top,
                        right: text_rect.right - POPUP_CELL_SIZE() / 2,
                        bottom: text_rect.bottom,
                    };
                    FillRect(hdc, &temp_rect, cell_brush);
                }

                SelectObject(hdc, old_brush);
            }
            // Draw middle cells
            else {
                FillRect(hdc, &text_rect, cell_brush);
            }

            // Draw selection
            if count == index {
                let old_brush = SelectObject(hdc, select_cell_brush);

                let margin = POPUP_CELL_SIZE() - POPUP_SELECT_CELL_SIZE();
                if POPUP_CIRCLE_SELECTION() {
                    Ellipse(
                        hdc,
                        text_rect.left + margin,
                        text_rect.top + margin,
                        text_rect.right - margin,
                        text_rect.bottom - margin,
                    );
                } else {
                    let select_rect = RECT {
                        left: text_rect.left + margin,
                        top: text_rect.top + margin,
                        right: text_rect.right - margin,
                        bottom: text_rect.bottom - margin,
                    };

                    RoundRect(
                        hdc,
                        select_rect.left,
                        select_rect.top,
                        select_rect.right,
                        select_rect.bottom,
                        POPUP_SELECT_CELL_ROUND(),
                        POPUP_SELECT_CELL_ROUND(),
                    );
                }

                SelectObject(hdc, old_brush);
            }

            // Draw text
            DrawTextExW(
                hdc,
                &mut u16_arr[count..=count],
                &mut text_rect,
                DT_CENTER | DT_SINGLELINE | DT_VCENTER,
                None,
            );

            count += 1;
        } // Cell loop end

        // Reset default objects
        SelectObject(hdc, old_font);
        SelectObject(hdc, old_palette);
        SelectObject(hdc, old_pen);
        // Delete created objects
        DeleteObject(font);
        DeleteObject(cell_brush);
        DeleteObject(select_cell_brush);
        DeleteObject(palette);
    }
}

fn get_popup_rect(cell_amount: i32) -> RECT {
    unsafe {
        // Get main monitor resolution
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);

        // 90% of height
        let bottom = height - height / 10;
        let top = bottom - POPUP_CELL_SIZE();
        let left: i32;
        let right: i32;

        if cell_amount % 2 == 0 {
            let center = width / 2;
            left = center - cell_amount / 2 * POPUP_CELL_SIZE();
            right = center + cell_amount / 2 * POPUP_CELL_SIZE();
        } else {
            let center = width / 2;
            left = center - POPUP_CELL_SIZE() / 2 - cell_amount / 2 * POPUP_CELL_SIZE();
            right = center + POPUP_CELL_SIZE() / 2 + cell_amount / 2 * POPUP_CELL_SIZE();
        }

        RECT {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }
}

fn get_palette() -> HPALETTE {
    unsafe {
        let cell = RGB::from_colorref(POPUP_CELL_COLOR);
        let select_cell = RGB::from_colorref(POPUP_SELECT_CELL_COLOR);
        let font = RGB::from_colorref(POPUP_FONT_COLOR);

        let palentry: [PALETTEENTRY; 3] = [
            // cell
            PALETTEENTRY {
                peRed: cell.r,
                peGreen: cell.g,
                peBlue: cell.b,
                peFlags: 0,
            },
            // font
            PALETTEENTRY {
                peRed: font.r,
                peGreen: font.g,
                peBlue: font.b,
                peFlags: 0,
            },
            // select cell
            PALETTEENTRY {
                peRed: select_cell.r,
                peGreen: select_cell.g,
                peBlue: select_cell.b,
                peFlags: 0,
            },
        ];
        CreatePalette(&LOGPALETTE {
            palVersion: 1337,
            palNumEntries: palentry.len() as u16,
            palPalEntry: std::mem::transmute(palentry[0]),
        })
    }
}

#[derive(PartialEq, Debug)]
struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    fn from_colorref(color: COLORREF) -> RGB {
        RGB {
            r: (color.0 & 0xFF) as u8,
            g: (color.0 >> 8 & 0xFF) as u8,
            b: (color.0 >> 16 & 0xFF) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popup_rgb_test() {
        // Test 1
        let mut color = COLORREF { 0: 0x0000DDFF };
        let mut result = RGB {
            r: 255,
            g: 221,
            b: 0,
        };
        assert_eq!(result, RGB::from_colorref(color));

        // Test 2
        color = COLORREF { 0: 0x00663F1E };
        result = RGB {
            r: 30,
            g: 63,
            b: 102,
        };
        assert_eq!(result, RGB::from_colorref(color));

        // Test 3
        color = COLORREF { 0: 0x00E8D2BC };
        result = RGB {
            r: 188,
            g: 210,
            b: 232,
        };
        assert_eq!(result, RGB::from_colorref(color));
    }
}
