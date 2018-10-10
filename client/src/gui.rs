extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background::Img, Color, Font, FontStyle, Image, PixelFormat},
    lifecycle::{Asset, Window},
};

struct MouseClick {
    x: f32, y: f32,
}
enum Event {
    MouseClick,
}

pub trait Widget {
    fn draw(&mut self, window: &mut Window); 
    fn event(&mut self, event: &Event); 
}
pub struct Gui{
    widgets: Vec<Box<Widget>>,
    focused_index: usize,
}
impl Gui{
    fn new() -> Gui{
        Gui{widgets:Vec::new(), focused_index:0}
    }
    fn gen<T:Widget + 'static>(&mut self, widget: T) {
        self.widgets.push(Box::new(widget));
    }
    fn draw(&mut self, window: &mut Window) {
        for w in self.widgets {
            w.draw(window);
        }
    }
    fn event(&mut self, event: Event) {
        for w in self.widgets {
            w.event(&event);
        }
    }
}



pub struct Label {
    x: f32, y: f32,
    font: Asset<Font>, style: FontStyle,
    image: Asset<Image>, text: String,
}
impl Label {
    fn new(x:f32, y:f32) -> Label {
        static default_font_asset: Asset<Font> = Asset::new(Font::load("font.ttf"));
        static default_font_style: FontStyle = FontStyle::new(10., Color::BLACK);
        Label{x:x, y:y, 
            font: default_font_asset, 
            style: default_font_style,
            image: Image::from_raw(&[], 1, 1, PixelFormat::RGBA).unwrap(),
            text: String::new(),
        }
    }
    fn style(&mut self, size:f32, color:Color) -> &mut Label {
        self.style = FontStyle::new(size, color);
        self.font.execute(|font| {
            self.image = font.render(&self.text, &self.style).unwrap();
            Ok(())
        });
        self
    }
    fn text(&mut self, text: &str) -> &mut Label {
        self.text = text.to_string();
        self.font.execute(|font| {
            self.image = font.render(&self.text, &self.style).unwrap();
            Ok(())
        });
        self
    }
}
impl Widget for Label {
    fn draw(&mut self, window: &mut Window) {
        window.draw(&self.image.area().with_center((self.x, self.y)), Img(&self.image));
    }
    fn event(&mut self, event: &Event) {

    }
}
/*
pub struct input {
    label: Label
}
impl Widget for input {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.draw(
    }
}
*/
