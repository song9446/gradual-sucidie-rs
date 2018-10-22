extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image, PixelFormat},
    lifecycle::{Asset, Window},
};
extern crate naive_gui;
use naive_gui::{
    Gui,
    Widget,
    Drawer,
};

pub struct QuickSilverDrawContext<'a> {
    font_size: f32,
    fill_color: Color,
    font_loader: Asset<Font>,
    //stroke_color: Color,
    window: &'a mut Window,
}
impl<'a> QuickSilverDrawContext<'a>{
    pub fn new(window: &'a mut Window) -> Self{
        let (r,g,b,a) = (0., 0., 0., 1.);
        QuickSilverDrawContext{
            fill_color: Color::BLACK,
            //stroke_color: (0., 0., 0., 1.),
            font_size: 12., 
            font_loader: Asset::new(Font::load("ttf/font.ttf")),
            window: window,
        }
    }
}
impl<'a> Drawer for QuickSilverDrawContext<'a>{
    fn set_fill_style(&mut self, rgba:(f32, f32, f32, f32)){
        self.fill_color = Color::BLACK;
    }
    fn set_stroke_style(&mut self, rgba:(f32, f32, f32, f32)){
        //self.stroke_rgba = rgba;
    }
    fn set_font_style(&mut self, size: f32) {
        self.font_size = size;
    }
    fn draw_rect(&mut self, xywh:(f32, f32, f32, f32)) {
        self.window.draw(&Rectangle::new((xywh.0, xywh.1), (xywh.2, xywh.3)), Col(self.fill_color));
    }
    fn draw_text(&mut self, text: &str, xy:(f32, f32)){
        let font_size = &mut self.font_size;
        let fill_color = &mut self.fill_color;
        let window = &mut self.window;
        self.font_loader.execute(|font| {
            println!("hi");
            let image = font.render(text, &FontStyle::new(*font_size, *fill_color)).unwrap();
            let rect = &image.area();
            (*window).draw(&image.area().translate((xy.0-image.area().x(), xy.1-image.area().y())), Img(&image));
            Ok(())
        });
    }
    fn rendered_text_wh(&mut self, text: &str) -> (f32, f32) {
        let mut wh = (0., 0.);
        let font_size = &mut self.font_size;
        let fill_color = &mut self.fill_color;
        self.font_loader.execute(|font| {
            let image = font.render(text, &FontStyle::new(*font_size, *fill_color)).unwrap();
            wh = (image.area().width(), image.area().height());
            Ok(())
        });
        wh
    }
}
