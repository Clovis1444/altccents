// build.rs
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_resource_file("resources/resources.rc");
    res.set_icon("resources/blue256.ico");

    res.compile().unwrap()
}
