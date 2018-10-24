extern crate slab;
use slab::Slab;
pub trait Drawer{
    fn set_fill_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_stroke_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_font_style(&mut self, size: f32);
    fn draw_rect(&mut self, xywh:(f32, f32, f32, f32));
    fn draw_text(&mut self, text: &str, xy:(f32, f32));
    fn rendered_text_wh(&mut self, text: &str) -> (f32, f32);
}
mod widget;
pub use self::widget::Widget;
pub use self::widget::Key;
pub struct Gui {
    widgets: Slab<Widget>,
}

impl Gui {
    pub fn new() -> Gui{
        Gui {
            widgets: Slab::new(),
        }
    }
    pub fn put(&mut self, widget:Widget) -> usize{
        self.widgets.insert(widget)
    }
    pub fn take(&mut self, widget_id: usize) -> Widget{
        self.widgets.remove(widget_id)
    }
    pub fn get(&self, widget_id: usize) -> &Widget{
        self.widgets.get(widget_id).unwrap()
    }
    pub fn get_mut(&mut self, widget_id: usize) -> &mut Widget{
        self.widgets.get_mut(widget_id).unwrap()
    }
    pub fn draw(&mut self, drawer:&mut Drawer) {
        for (_, widget) in self.widgets.iter() {
            widget.draw(drawer);
        }
    }
    pub fn mouse_down(&mut self, xy:(f32,f32)){
        for (_, mut widget) in self.widgets.iter_mut() {
            widget.mouse_down(xy);
        }
    }
    pub fn key_down(&mut self, key: Key) {
        for (_, mut widget) in self.widgets.iter_mut() {
            widget.key_down(key);
        }
    }
    pub fn key_input(&mut self, ch: char) {
        for (_, mut widget) in self.widgets.iter_mut() {
            widget.key_input(ch);
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
