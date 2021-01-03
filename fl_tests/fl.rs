
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;

pub struct UserInterface{
    pub widget_0: Window,
    pub but: Button,
}

impl UserInterface{
    pub fn make_window()-> Self {
	let mut widget_0 = Window::new(138, 161, 440, 355, "");
	widget_0.show();
	let mut but = Button::new(175, 230, 95, 45, "Click me");
    Self { widget_0, but, }
    }
}
