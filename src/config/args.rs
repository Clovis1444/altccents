//! # args
//! `args.rs` contains all CLI arguments and how to handle them.
//!
//! # Add new argument
//! To add new argument do the following steps:
//! 1. Create new function for handling the argument.
//! 2. Create new function for getting the argument from `config::Settings`.
//! 3. Add new item to `SETTING_ARG_LIST`. Item must contain **argument name** and **pointers to functions** from previous steps from the first step.

struct SettingArg<'a> {
    name: &'a str,
    handler: fn(&str, &str),
    from_settings: fn(&str) -> Option<String>,
}

/// Array of all args and pointers to their handler functions.
const SETTING_ARG_LIST: [SettingArg; 14] = [
    SettingArg {
        name: "controlkey",
        handler: SettingArg::arg_controlkey,
        from_settings: SettingArg::get_arg_controlkey,
    },
    SettingArg {
        name: "timer",
        handler: SettingArg::arg_timer,
        from_settings: SettingArg::get_arg_timer,
    },
    SettingArg {
        name: "notimer",
        handler: SettingArg::arg_notimer,
        from_settings: SettingArg::get_arg_timer,
    },
    SettingArg {
        name: "sound",
        handler: SettingArg::arg_sound,
        from_settings: SettingArg::get_arg_sound,
    },
    SettingArg {
        name: "nosound",
        handler: SettingArg::arg_nosound,
        from_settings: SettingArg::get_arg_sound,
    },
    SettingArg {
        name: "enable",
        handler: SettingArg::arg_enable,
        from_settings: SettingArg::get_arg_status,
    },
    SettingArg {
        name: "disable",
        handler: SettingArg::arg_disable,
        from_settings: SettingArg::get_arg_status,
    },
    SettingArg {
        name: "transp",
        handler: SettingArg::arg_transp,
        from_settings: SettingArg::get_arg_transp,
    },
    SettingArg {
        name: "fontsize",
        handler: SettingArg::arg_fontsize,
        from_settings: SettingArg::get_arg_fontsize,
    },
    SettingArg {
        name: "cellsize",
        handler: SettingArg::arg_cellsize,
        from_settings: SettingArg::get_arg_cellsize,
    },
    SettingArg {
        name: "circleselect",
        handler: SettingArg::arg_circleselect,
        from_settings: SettingArg::get_arg_select,
    },
    SettingArg {
        name: "rectselect",
        handler: SettingArg::arg_rectselect,
        from_settings: SettingArg::get_arg_select,
    },
    SettingArg {
        name: "selectscale",
        handler: SettingArg::arg_selectscale,
        from_settings: SettingArg::get_arg_selectscale,
    },
    SettingArg {
        name: "round",
        handler: SettingArg::arg_round,
        from_settings: SettingArg::get_arg_round,
    },
    // Insert new arg here
];

/// This function will call arg handler function if the valid arg was provided.
pub fn validate_arg(arg: &str) {
    for i in SETTING_ARG_LIST {
        let arg = arg.to_lowercase();
        if arg.starts_with(i.name) {
            // (i.handler)(&arg, i.name);
            i.validate(&arg);
            return;
        }
    }
}

/// This function returns String of arguments depends on the current `SETTINGS`
pub fn get_args() -> String {
    let mut args = String::new();
    for i in SETTING_ARG_LIST {
        match i.get_arg() {
            Some(arg) => {
                args.push_str(&arg);
                args.push(' ');
            }
            None => (),
        }
    }

    args
}

impl SettingArg<'_> {
    fn get_arg(&self) -> Option<String> {
        (self.from_settings)(self.name)
    }
    fn validate(&self, arg: &str) {
        (self.handler)(arg, self.name)
    }

    //
    //
    //
    fn arg_controlkey(arg: &str, name: &str) {
        use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let key = match val.parse::<u16>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    let key = VIRTUAL_KEY { 0: key };
                    if super::Settings::VALID_CONTROL_KEYS.contains(&key) {
                        super::SETTINGS.control_key = key;
                    }
                }
                None => (),
            }
        }
    }
    fn get_arg_controlkey(arg: &str) -> Option<String> {
        let mut arg = arg.to_lowercase();
        arg.push_str(&super::CONTROL_KEY().0.to_string());
        Some(arg)
    }

    fn arg_timer(arg: &str, name: &str) {
        unsafe {
            super::SETTINGS.use_timer = true;

            match arg.strip_prefix(name) {
                Some(val) => {
                    let key_interval = match val.parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.max_key_interval = key_interval;
                }
                None => (),
            };
        }
    }
    fn arg_notimer(_arg: &str, _name: &str) {
        unsafe {
            super::SETTINGS.use_timer = false;
        }
    }
    fn get_arg_timer(arg: &str) -> Option<String> {
        match super::USE_TIMER() {
            true => {
                if arg.to_lowercase() == "notimer" {
                    return None;
                }
                let mut arg = arg.to_lowercase();
                arg.push_str(&super::MAX_KEY_INTERVAL().to_string());
                Some(arg)
            }
            false => {
                if arg.to_lowercase() == "timer" {
                    return None;
                }
                Some("notimer".to_string())
            }
        }
    }

    fn arg_sound(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.use_sound = true }
    }
    fn arg_nosound(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.use_sound = false }
    }
    fn get_arg_sound(arg: &str) -> Option<String> {
        unsafe {
            match super::SETTINGS.use_sound {
                true => {
                    if arg.to_lowercase() == "nosound" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
                false => {
                    if arg.to_lowercase() == "sound" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
            }
        }
    }

    fn arg_enable(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.default_program_status = true }
    }
    fn arg_disable(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.default_program_status = false }
    }
    fn get_arg_status(arg: &str) -> Option<String> {
        unsafe {
            match super::SETTINGS.default_program_status {
                true => {
                    if arg.to_lowercase() == "disable" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
                false => {
                    if arg.to_lowercase() == "enable" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
            }
        }
    }

    fn arg_transp(arg: &str, name: &str) {
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let transp = match val.parse::<u8>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.popup_window_transparency = transp;

                    use super::{super::session::PROGRAM_DATA, *};
                    use windows::Win32::UI::WindowsAndMessaging::{
                        SetLayeredWindowAttributes, LWA_ALPHA, LWA_COLORKEY,
                    };

                    if let Some(hwnd) = PROGRAM_DATA.get_hwnd_option() {
                        let _ = SetLayeredWindowAttributes(
                            hwnd,
                            POPUP_WINDOW_TRANSPARENT_COLOR,
                            POPUP_WINDOW_TRANSPARENCY(),
                            LWA_ALPHA | LWA_COLORKEY,
                        );
                    }
                }
                None => (),
            };
        }
    }
    fn get_arg_transp(arg: &str) -> Option<String> {
        unsafe {
            let mut arg = arg.to_lowercase();
            arg.push_str(&super::SETTINGS.popup_window_transparency.to_string());
            Some(arg)
        }
    }

    fn arg_fontsize(arg: &str, name: &str) {
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let font_size = match val.parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.popup_font_size = font_size as i32;
                }
                None => (),
            };
        }
    }
    fn get_arg_fontsize(arg: &str) -> Option<String> {
        unsafe {
            let mut arg = arg.to_lowercase();
            arg.push_str(&super::SETTINGS.popup_font_size.to_string());
            Some(arg)
        }
    }

    fn arg_cellsize(arg: &str, name: &str) {
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let cell_size = match val.parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.popup_cell_size = cell_size as i32;
                }
                None => (),
            };
        }
    }
    fn get_arg_cellsize(arg: &str) -> Option<String> {
        unsafe {
            let mut arg = arg.to_lowercase();
            arg.push_str(&super::SETTINGS.popup_cell_size.to_string());
            Some(arg)
        }
    }

    fn arg_circleselect(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.popup_circle_selection = true }
    }
    fn arg_rectselect(_arg: &str, _name: &str) {
        unsafe { super::SETTINGS.popup_circle_selection = false }
    }
    fn get_arg_select(arg: &str) -> Option<String> {
        unsafe {
            match super::SETTINGS.popup_circle_selection {
                true => {
                    if arg.to_lowercase() == "rectselect" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
                false => {
                    if arg.to_lowercase() == "circleselect" {
                        return None;
                    }
                    Some(arg.to_lowercase())
                }
            }
        }
    }

    fn arg_selectscale(arg: &str, name: &str) {
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let scale = match val.parse::<f32>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.popup_select_cell_scale = scale;
                }
                None => (),
            };
        }
    }
    fn get_arg_selectscale(arg: &str) -> Option<String> {
        unsafe {
            let mut arg = arg.to_lowercase();
            arg.push_str(&super::SETTINGS.popup_select_cell_scale.to_string());
            Some(arg)
        }
    }

    fn arg_round(arg: &str, name: &str) {
        unsafe {
            match arg.strip_prefix(name) {
                Some(val) => {
                    let round = match val.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => return,
                    };

                    super::SETTINGS.popup_round_factor = round;
                }
                None => (),
            };
        }
    }
    fn get_arg_round(arg: &str) -> Option<String> {
        unsafe {
            let mut arg = arg.to_lowercase();
            arg.push_str(&super::SETTINGS.popup_round_factor.to_string());
            Some(arg)
        }
    }

    // Insert new argument functions here
}
