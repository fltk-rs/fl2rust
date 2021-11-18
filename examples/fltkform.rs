// this is to build the configuration
use fltk::{prelude::*, *};
#[macro_use]
extern crate fltk_form_derive;
use fltk_form::{FltkForm, HasProps};

#[path = "mockui.rs"]
mod mockui;

// Emit status
#[derive(Copy, Clone)]
enum Status {
    Update,
    Quit,
}
// cities
#[derive(Debug, Clone, Copy, FltkForm)]
enum City {
    Watonga,
    Shawnee,
    Pawhuska,
    None,
}
// states
#[derive(Debug, Clone, Copy, FltkForm)]
enum State {
    Oklahoma,
    None,
}

#[derive(Debug, Clone, FltkForm)]
struct Contact {
  name:String,
  phone:u32,
  address1:String,
  address2:String,
  city:City,
  state:State,
}
impl Contact {
    pub fn new() -> Self {
        Contact {
            name:String::from(""),
            phone:0,
            address1:String::from(""),
            address2:String::from(""),
            city:City::None,
            state:State::None,
        }
    }
}

fn main() {
    let app = app::App::default();
    let (send_action, receive_action) = app::channel::<Status>();
    let mut ui = mockui::UserInterface::make_window();
    ui.win.show();

    // make clicking either tab emit signal
    ui.tabs.emit(send_action, Status::Update);

    let mut page1 = Contact::new();
    //update ui page 1
    ui.page_1.begin();
    let inner_content = page1.generate();
    ui.page_1.end();

    let mut page2 = Contact::new();
    //update ui page 1
    ui.page_2.begin();
    let inner_content = page1.generate();
    ui.page_2.end();
    while app.wait(){
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Status::Update => {
                    // get tab clicked
                    //TODO
                },
                Status::Quit => {
                    app.quit();
                },
            }
        }
    }
}
