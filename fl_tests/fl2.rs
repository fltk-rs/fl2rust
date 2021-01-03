
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
    pub dw: Window,
    pub but: Button,
    pub inp: Input,
    pub gr: Group,
    pub but2: Button,
    pub inp2: Input,
}

impl UserInterface{
    pub fn make_window() -> Self {
	let mut dw = Window::new(411, 233, 620, 370, "");
	dw.end();
	dw.show();
	let mut but = Button::new(395, 85, 105, 45, "");
	dw.add(&but);
	let mut inp = Input::new(195, 85, 185, 45, "input: adfa rewar");
	inp.set_label_size(32);
	inp.set_text_size(32);
	dw.add(&inp);
	let mut gr = Group::new(411, 233, 620, 370, "");
	gr.end();
	gr.show();
	dw.add(&gr);
	let mut but2 = Button::new(395, 85, 105, 45, "");
	gr.add(&but2);
	let mut inp2 = Input::new(195, 85, 185, 45, "input: adfa rewar");
	inp2.set_label_size(32);
	inp2.set_text_size(32);
	dw.add(&inp2);
    Self { dw, but, inp, gr, but2, inp2, }
    }
}
