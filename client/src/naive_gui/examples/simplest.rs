extern crate naive_gui;
use naive_gui::{
    Gui,
    Drawer,
    Widget::*,
};
struct DrawContext {
    fill_rgba: (f32, f32, f32, f32),
    stroke_rgba: (f32, f32, f32, f32),
    font_size: f32,
}
impl DrawContext {
    fn new() -> Self{
        DrawContext{
            fill_rgba: (0., 0., 0., 1.),
            stroke_rgba: (0., 0., 0., 1.),
            font_size: 12.,
        }
    }
}
impl Drawer for DrawContext{
    fn set_fill_style(&mut self, rgba:(f32, f32, f32, f32)){
        self.fill_rgba = rgba;
    }
    fn set_stroke_style(&mut self, rgba:(f32, f32, f32, f32)){
        self.stroke_rgba = rgba;
    }
    fn set_font_style(&mut self, size: f32) {
        self.font_size = size;
    }
    fn draw_rect(&self, xywh:(f32, f32, f32, f32)) {
        println!("draw rect at {:?}", xywh);
    }
    fn draw_text(&self, text: &str, xy:(f32, f32)){
        println!("draw text({}) at {}, {}", text, xy.0, xy.1);
    }
}
fn main(){
    let mut drawer = DrawContext::new();
    let mut gui = Gui::new();
    let a = gui.gen(Label{text:"hi".to_string(), size:12., xy:(1., 1.), rgba:(1.,1.,1.,0.)});
    gui.gen(Label{text:"hello".to_string(), size:12., xy:(1., 1.), rgba:(1.,1.,1.,0.)});
    if let Label{ref text, ..} = a {
        println!("{}?", text);
    }
    gui.draw(&mut drawer);
}
