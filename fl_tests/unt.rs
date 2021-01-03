
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
    pub widget_1: Window,
    pub widget_2: MenuBar,
}

impl UserInterface{
    pub fn make_window()-> Self {
	let mut widget_1 = Window::new(29, 82, 469, 333, "");
	widget_1.end();
	widget_1.show();
	let mut widget_2 = MenuBar::new(0, 0, 350, 35, "");
	widget_2.end();
	widget_1.add(&widget_2);
	widget_2.add("item", Shortcut::None, MenuFlag::Normal, || {});
    Self { widget_1, widget_2, }
    }
}
