// data.rs

use windows::Win32::UI::{
    Input::KeyboardAndMouse::*,
    WindowsAndMessaging::{MSG, WM_KEYDOWN},
};

// Main accent data. Order of AccentChar's and AccentKey variants MUST MATCH!
const ACCENT_LIST: [AccentChar<'_>; AccentKey::EnumLength as usize] = [
    // 0 = AccentKey::A
    AccentChar {
        lower_case: &['à', 'á', 'â', 'ä', 'æ'],
        upper_case: &['À', 'Á', 'Â', 'Ä', 'Æ'],
    },
    // 1 = AccentKey::E
    AccentChar {
        lower_case: &['é', 'è', 'ê', 'ë'],
        upper_case: &['É', 'È', 'Ê', 'Ë'],
    },
    // 2 = AccentKey::I
    AccentChar {
        lower_case: &['î', 'ï'],
        upper_case: &['Î', 'Ï'],
    },
    // 3 = AccentKey::O
    AccentChar {
        lower_case: &['ô', 'ö', 'œ'],
        upper_case: &['Ô', 'Ö', 'Œ'],
    },
    // 4 = AccentKey::U
    AccentChar {
        lower_case: &['ù', 'û', 'ü'],
        upper_case: &['Ù', 'Û', 'Ü'],
    },
    // 5 = AccentKey::C
    AccentChar {
        lower_case: &['ç'],
        upper_case: &['Ç'],
    },
    // 6 = AccentKey::Y
    AccentChar {
        lower_case: &['ÿ'],
        upper_case: &['Ÿ'],
    },
    // 7 = AccentKey::Euro
    AccentChar {
        lower_case: &['€'],
        upper_case: &[],
    },
];

pub fn get_accent(key: AccentKey, is_capital: bool, index: usize) -> char {
    if let AccentKey::EnumLength = key {
        return '?';
    }

    let ch = &ACCENT_LIST[key as usize];

    match is_capital {
        false => return *ch.lower_case.get(index).unwrap_or(&'?'),
        true => {
            if ch.upper_case.is_empty() {
                return *ch.lower_case.get(index).unwrap_or(&'?');
            } else {
                return *ch.upper_case.get(index).unwrap_or(&'?');
            }
        }
    }
}

pub fn get_accent_chars(key: AccentKey, is_capital: bool) -> &'static [char] {
    if let AccentKey::EnumLength = key {
        return &['?'];
    }

    let accent = &ACCENT_LIST[key as usize];

    match is_capital {
        false => accent.lower_case,
        true => {
            if accent.upper_case.is_empty() {
                return accent.lower_case;
            } else {
                return accent.upper_case;
            }
        }
    }
}

pub fn accent_amount(key: &AccentKey) -> Option<usize> {
    if let AccentKey::EnumLength = key {
        return None;
    }

    Some(ACCENT_LIST[*key as usize].lower_case.len())
}

struct AccentChar<'a> {
    pub lower_case: &'a [char],
    pub upper_case: &'a [char],
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AccentKey {
    A,
    E,
    I,
    O,
    U,
    C,
    Y,
    Euro,
    // Isert new keys here
    EnumLength,
}

impl AccentKey {
    const MAPPED_KEYS: [(AccentKey, VIRTUAL_KEY); AccentKey::EnumLength as usize] = [
        (AccentKey::A, VK_A),
        (AccentKey::E, VK_E),
        (AccentKey::I, VK_I),
        (AccentKey::O, VK_O),
        (AccentKey::U, VK_U),
        (AccentKey::C, VK_C),
        (AccentKey::Y, VK_Y),
        (AccentKey::Euro, VK_OEM_7),
    ];

    #[allow(dead_code)]
    pub fn vk(&self) -> Option<VIRTUAL_KEY> {
        for (ak, vk) in AccentKey::MAPPED_KEYS {
            if *self == ak {
                return Some(vk);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn from_msg(msg: &MSG) -> Option<AccentKey> {
        if msg.message != WM_KEYDOWN {
            return None;
        }

        let key = msg.wParam.0 as u16;

        for (ak, vk) in AccentKey::MAPPED_KEYS {
            if key == vk.0 {
                return Some(ak);
            }
        }

        None
    }

    pub fn from_vk(virtual_key: &VIRTUAL_KEY) -> Option<AccentKey> {
        for (ak, vk) in AccentKey::MAPPED_KEYS {
            if *virtual_key == vk {
                return Some(ak);
            }
        }

        None
    }
}
