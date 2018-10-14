extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background::Img, Color, Font, FontStyle, Image, PixelFormat},
    lifecycle::{Asset, Window},
};

#[derive(Clone)]
struct MouseClick {
    x: f32, y: f32,
}
#[derive(Clone)]
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
        for w in self.widgets.iter() {
            w.draw(window);
        }
    }
    fn event(&mut self, event: &Event) {
        for w in self.widgets {
            w.event(&event);
        }
    }
}

pub struct Label {
    x: f32, y: f32, style: FontStyle,
    image: Image, text: String, updated: bool,
}
impl Label {
    fn new(x:f32, y:f32, style:FontStyle) -> Label {
        Label{x:x, y:y, 
            font: font, 
            style: style,
            image: Image::from_raw(&[], 1, 1, PixelFormat::RGBA).unwrap(),
            text: String::new(),
            updated: true,
        }
    }
    fn update(&mut self) {
        let mut image = &mut self.image;
        let text = &self.text;
        let style = &self.style;
        self.font.execute(|font| {
            *image = font.render(text, style).unwrap();
            Ok(())
        });
    }
    fn style(&mut self, size:f32, color:Color) -> &mut Label {
        self.style = FontStyle::new(size, color);
        self
    }
    fn text(&mut self, text: &str) -> &mut Label {
        self.text = text.to_string();
        self
    }
}
impl<'a> Widget for Label<'a> {
    fn draw(&mut self, window: &mut Window) {
        self.update();
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
