// args.rs
// To add new argument you need to create new item in SETTING_ARG_LIST: pass arg name to SettingArg and function pointer with argument logic

struct SettingArg<'a> {
    name: &'a str,
    function: fn(&str, &str),
}

const SETTING_ARG_LIST: [SettingArg; 14] = [
    SettingArg {
        name: "controlkey",
        function: arg_controlkey,
    },
    SettingArg {
        name: "timer",
        function: arg_timer,
    },
    SettingArg {
        name: "nosound",
        function: arg_nosound,
    },
    SettingArg {
        name: "disable",
        function: arg_disable,
    },
    SettingArg {
        name: "fontsize",
        function: arg_fontsize,
    },
    SettingArg {
        name: "transp",
        function: arg_transp,
    },
    SettingArg {
        name: "cellsize",
        function: arg_cellsize,
    },
    SettingArg {
        name: "circleselect",
        function: arg_circleselect,
    },
    SettingArg {
        name: "enable",
        function: arg_enable,
    },
    SettingArg {
        name: "sound",
        function: arg_sound,
    },
    SettingArg {
        name: "notimer",
        function: arg_notimer,
    },
    SettingArg {
        name: "rectselect",
        function: arg_rectselect,
    },
    SettingArg {
        name: "selectscale",
        function: arg_selectscale,
    },
    SettingArg {
        name: "round",
        function: arg_round,
    },
    // TODO: add "round", "selectscale"; "nosound"/"off"/"circle"/"notimer" oposites
    // Add readme entries
    // Insert new arg here
];

pub fn validate_arg(arg: &str) {
    for i in SETTING_ARG_LIST {
        let arg = arg.to_lowercase();
        if arg.starts_with(i.name) {
            (i.function)(&arg, i.name);
            return;
        }
    }
}

//
//
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
fn arg_nosound(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.use_sound = false }
}
fn arg_disable(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.default_program_status = false }
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
fn arg_transp(arg: &str, name: &str) {
    unsafe {
        match arg.strip_prefix(name) {
            Some(val) => {
                let transp = match val.parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => return,
                };

                super::SETTINGS.popup_window_transparency = transp;
            }
            None => (),
        };
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
fn arg_circleselect(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.popup_circle_selection = true }
}
fn arg_enable(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.default_program_status = true }
}
fn arg_sound(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.use_sound = true }
}
fn arg_notimer(_arg: &str, _name: &str) {
    unsafe {
        super::SETTINGS.use_timer = false;
    }
}
fn arg_rectselect(_arg: &str, _name: &str) {
    unsafe { super::SETTINGS.popup_circle_selection = false }
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
// Insert new argument validation function here
