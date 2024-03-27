// build.rs
#[cfg(windows)]
extern crate windres;

fn main() {
    windres::Build::new()
        .compile("resources/resources.rc")
        .unwrap();
}
