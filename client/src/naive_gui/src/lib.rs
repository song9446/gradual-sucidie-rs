pub trait Drawer{
    fn set_fill_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_stroke_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_font_style(&mut self, size: f32);
    fn draw_rect(&self, xywh:(f32, f32, f32, f32));
    fn draw_text(&self, text: &str, xy:(f32, f32));
}
mod widget;
pub use self::widget::Widget;
pub struct Gui {
    widgets: Vec<Widget>,
}
impl Gui {
    pub fn new() -> Gui{
        Gui {
            widgets:Vec::new(),
        }
    }
    pub fn gen(&mut self, widget:Widget) -> &mut Widget{
        self.widgets.push(widget);
        self.widgets.last_mut().unwrap()
    }
    pub fn draw(&self, drawer:&mut Drawer) {
        for widget in self.widgets.iter() {
            widget.draw(drawer);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
