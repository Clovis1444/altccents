// data.rs

use windows::Win32::UI::Input::KeyboardAndMouse::*;

// Main accent data. Order of AccentChar's and AccentKey variants MUST MATCH!
// Get info: use get_accent()
const ACCENT_LIST: [AccentChar<'_>; AccentKey::MaxKey as usize] = [
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

pub fn get_accent(key: AccentKey, capital: bool, index: usize) -> char {
    if let AccentKey::MaxKey = key {
        return '?';
    }

    let ch = &ACCENT_LIST[key as usize];

    match capital {
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

struct AccentChar<'a> {
    pub lower_case: &'a [char],
    pub upper_case: &'a [char],
}

// struct AccentList {}

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
    MaxKey,
}

impl AccentKey {
    pub fn vk(&self) -> Option<VIRTUAL_KEY> {
        match self {
            AccentKey::A => Some(VK_A),
            AccentKey::E => Some(VK_E),
            AccentKey::I => Some(VK_I),
            AccentKey::O => Some(VK_O),
            AccentKey::U => Some(VK_U),
            AccentKey::C => Some(VK_C),
            AccentKey::Y => Some(VK_Y),
            // " ' " or "Э" key
            AccentKey::Euro => Some(VK_OEM_7),
            _ => None,
        }
    }
}
