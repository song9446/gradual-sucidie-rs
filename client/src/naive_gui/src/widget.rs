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
use ::Drawer; 
pub impl Widget {
    fn draw(&self, drawer:&Drawer){
        match &self {
            Label{text, size, xy, rgba} => {
                drawer.set_font_style(size);
                drawer.set_fill_color(rgba);
                drawer.draw_text(text, size, xy);
            }
        }
    }
}
