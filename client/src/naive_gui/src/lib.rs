pub trait Drawer {
    pub fn set_fill_style(rgba:(f32, f32, f32, f32));
    pub fn set_stroke_style(rgba:(f32, f32, f32, f32));
    pub fn set_font_style(size: f32);
    pub fn draw_rect(xywh:(f32, f32, f32, f32));
    pub fn draw_text(text: &str, xy:(f32, f32));
}
mod widget;
pub use widget::Widgets;
pub struct Gui {
    widgets: Vec<Widget>,
}
impl Gui {
    pub fn gen(widget:Widget) {
        widgets.push(widget);
    }
    pub fn draw(drawer:&Drawer) {
        for widget in widgets {
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
