use fltk::{prelude::*, *};

mod ui {
    fl2rust_macro::include_ui!("src/myui.fl");
}

fn main() {
    let a = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    ui.but.set_callback(|b| println!("Button clicked!"));
    a.run().unwrap();
}
