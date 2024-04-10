// build.rs
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();

    res.append_rc_content(
        r##"
    1 FONT "resources/InterVariable.ttf"
    PROGRAM_ICON ICON "resources/program_icon.ico"
    PROGRAM_ON_ICON ICON "resources/program_icon_on.ico"
    PROGRAM_OFF_ICON ICON "resources/program_icon_off.ico"
    "##,
    );

    res.set_icon("resources/program_icon.ico");
    res.set("FileDescription", "Utility for typing accented characters")
        .set("LegalCopyright", "Copyright (C) 2024");

    res.compile().unwrap();
}
