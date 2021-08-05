use fltk::{prelude::*, *};

#[path = "myuifile.rs"]
mod myuifile;

fn main() {
    let app = app::App::default();
    let _ui = myuifile::UserInterface::make_window();
    app.run().unwrap();
}