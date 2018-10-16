pub enum Widget {
    Label{
        text:String, 
        size:f32,
        xy:(f32, f32),  
        rgba:(f32, f32, f32, f32),
    },
    Input{
        text:String, 
        size:f32,
        xy:(f32, f32),  
        rgba:(f32, f32, f32, f32),
    },
}
use super::Drawer; 
impl Widget {
    pub fn draw(&self, drawer:&mut Drawer){
        match &self {
            Widget::Label{text, size, xy, rgba} => {
                drawer.set_font_style(*size);
                drawer.set_fill_style(*rgba);
                drawer.draw_text(text, *xy);
            }
            _ => {
            }
        }
    }
}
