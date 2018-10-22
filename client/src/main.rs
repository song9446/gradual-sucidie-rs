#[macro_use]
extern crate stdweb;
extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};

extern crate naive_gui;
use naive_gui::{
    Gui,
    Widget,
    Drawer,
};
mod gui;
use self::gui::QuickSilverDrawContext;

struct Game {
    gui: Gui,
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut gui = Gui::new();
        let id = gui.put(Widget::Label{text:"hi".to_string(), size:12., xy:(1., 1.), rgba:(1.,1.,1.,0.)});
        Ok(
            Game{
                gui: gui})
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        let mut dc = QuickSilverDrawContext::new(window); 
        self.gui.draw(&mut dc);
        Ok(())
    }
}

fn main() {
    run::<Game>("Font Example", Vector::new(800, 600), Settings::default());
}
