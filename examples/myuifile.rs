// Automatically generated by fl2rust

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::needless_update)]

use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
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



#[derive(Debug, Clone, Default)]
pub struct UserInterface {
}


impl UserInterface {
    pub fn make_window() -> Self {
        let mut fl2rust_widget_0 = Window::new(616, 196, 628, 505, None);
        fl2rust_widget_0.end();
        fl2rust_widget_0.show();
        let mut fl2rust_widget_1 = Tabs::new(4, 10, 634, 515, None);
        fl2rust_widget_1.end();
        fl2rust_widget_0.add(&fl2rust_widget_1);
        let mut fl2rust_widget_2 = Group::new(5, 30, 620, 480, "Tab1");
        fl2rust_widget_2.end();
        fl2rust_widget_1.add(&fl2rust_widget_2);
        let mut fl2rust_widget_3 = Button::new(30, 56, 119, 32, "button");
        fl2rust_widget_2.add(&fl2rust_widget_3);
        let mut fl2rust_widget_4 = RoundButton::new(30, 99, 119, 30, "Round");
        fl2rust_widget_4.set_down_frame(FrameType::RoundDownBox);
        fl2rust_widget_2.add(&fl2rust_widget_4);
        let mut fl2rust_widget_5 = CheckButton::new(30, 141, 119, 30, "Check");
        fl2rust_widget_5.set_down_frame(FrameType::DownBox);
        fl2rust_widget_2.add(&fl2rust_widget_5);
        let mut fl2rust_widget_6 = LightButton::new(30, 182, 119, 30, "Light");
        fl2rust_widget_2.add(&fl2rust_widget_6);
        let mut fl2rust_widget_7 = MenuButton::new(30, 224, 119, 30, "menu");
        fl2rust_widget_7.end();
        fl2rust_widget_2.add(&fl2rust_widget_7);
        let mut fl2rust_widget_8 = ReturnButton::new(30, 265, 119, 30, "Return");
        fl2rust_widget_2.add(&fl2rust_widget_8);
        let mut fl2rust_widget_9 = Choice::new(30, 307, 119, 30, "Choice");
        fl2rust_widget_9.end();
        fl2rust_widget_9.set_down_frame(FrameType::BorderBox);
        fl2rust_widget_9.set_align(unsafe {std::mem::transmute(16)});
        fl2rust_widget_2.add(&fl2rust_widget_9);
        let mut fl2rust_widget_10 = Input::new(30, 348, 119, 30, None);
        fl2rust_widget_2.add(&fl2rust_widget_10);
        let mut fl2rust_widget_11 = Output::new(30, 390, 119, 30, None);
        fl2rust_widget_2.add(&fl2rust_widget_11);
        let mut fl2rust_widget_12 = Counter::new(30, 438, 120, 22, "counter:");
        fl2rust_widget_2.add(&fl2rust_widget_12);
        let mut fl2rust_widget_13 = Slider::new(180, 55, 25, 331, "slider:");
        fl2rust_widget_13.set_type(SliderType::VerticalNice);
        fl2rust_widget_2.add(&fl2rust_widget_13);
        let mut fl2rust_widget_14 = Scrollbar::new(253, 56, 23, 331, "Scrollbar");
        fl2rust_widget_2.add(&fl2rust_widget_14);
        let mut fl2rust_widget_15 = Roller::new(315, 59, 19, 331, "Roller");
        fl2rust_widget_2.add(&fl2rust_widget_15);
        let mut fl2rust_widget_16 = Dial::new(405, 76, 155, 139, "Dial");
        fl2rust_widget_16.set_type(DialType::Line);
        fl2rust_widget_16.set_color(Color::by_index(183));
        fl2rust_widget_2.add(&fl2rust_widget_16);
        let mut fl2rust_widget_17 = Scrollbar::new(180, 440, 415, 20, None);
        fl2rust_widget_17.set_type(ScrollbarType::Horizontal);
        fl2rust_widget_2.add(&fl2rust_widget_17);
        let mut fl2rust_widget_18 = FileBrowser::new(390, 240, 190, 150, None);
        fl2rust_widget_18.set_type(BrowserType::Multi);
        fl2rust_widget_2.add(&fl2rust_widget_18);
        let mut fl2rust_widget_19 = Group::new(30, 35, 600, 470, "Tab2");
        fl2rust_widget_19.end();
        fl2rust_widget_19.hide();
        fl2rust_widget_1.add(&fl2rust_widget_19);
        let mut fl2rust_widget_20 = Group::new(20, 36, 605, 468, "Tab3");
        fl2rust_widget_20.end();
        fl2rust_widget_20.hide();
        fl2rust_widget_1.add(&fl2rust_widget_20);
        Self { }
    }
}
