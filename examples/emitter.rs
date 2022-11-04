// this is to build the configuration
use fltk::{prelude::*, *};
#[macro_use]
extern crate fltk_form_derive;
use fltk_form::{FltkForm, HasProps};

#[path = "emitui.rs"]
mod emitui;

// Emit status
#[derive(Copy, Clone)]
enum Action {
    Open,
    Clear,
    Quit,
}

fn main() {
    let app = app::App::default();
    let mut buf = text::TextBuffer::default();
    let mut editor = text::TextEditor::new(15, 55, 400, 260, "");
    let (send_action, receive_action) = app::channel::<Action>();
    let mut ui = emitui::UserInterface::make_window();
    ui.win.show();
    // make  emit ui..emit(send_action, Action::);
    // make open_button emit Open
    ui.open_button.emit(send_action, Action::Open);
    // make  clear_button emit Clear
    ui.clear_button.emit(send_action, Action::Clear);
    while app.wait() {
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Action::Open => {
                    let file = match dialog::file_chooser("Choose File", "*", ".", true) {
                        Some(file) => file,
                        None => continue,
                    };
                    match buf.load_file(&file) {
                        Ok(_i) => {
                            ui.filename.set_value(file.as_str());
                            // update the scroll box
                            ui.scroll.begin();
                            // new editor
                            editor = text::TextEditor::new(15, 55, 400, 260, "");
                            // set the bufffer
                            editor.set_buffer(Some(buf.clone()));
                            // end the group again
                            ui.scroll.end();
                        }
                        Err(e) => {
                            println!("Buffer Error:{}", e);
                        }
                    };
                }
                Action::Clear => {
                    ui.filename.set_value("");
                    ui.scroll.clear();
                }
                Action::Quit => {
                    app.quit();
                }
            }
        }
        ui.win.redraw();
    }
}
