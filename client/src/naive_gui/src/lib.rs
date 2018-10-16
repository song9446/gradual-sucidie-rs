extern crate slab;
use slab::Slab;
pub trait Drawer{
    fn set_fill_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_stroke_style(&mut self, rgba:(f32, f32, f32, f32));
    fn set_font_style(&mut self, size: f32);
    fn draw_rect(&self, xywh:(f32, f32, f32, f32));
    fn draw_text(&self, text: &str, xy:(f32, f32));
    fn rendered_text_wh(&self, text: &str) -> (f32, f32);
}
mod widget;
pub use self::widget::Widget;
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
    pub fn get(&mut self, widget_id: usize) -> &Widget{
        self.widgets.get(widget_id).unwrap()
    }
    pub fn get_mut(&mut self, widget_id: usize) -> &mut Widget{
        self.widgets.get_mut(widget_id).unwrap()
    }
    pub fn draw(&self, drawer:&mut Drawer) {
        for (_, widget) in self.widgets.iter() {
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
