pub enum Widget {
    Label{
        text:String, 
        size:f32,
        xy:(f32, f32),  
        rgba:(f32, f32, f32, f32),
    },
    Input{
        focused: bool,
        text:String, 
        size:f32,
        xy:(f32, f32),  
        wh:(f32, f32),
        rgba:(f32, f32, f32, f32),
    },
}
use super::Drawer; 
use std::time::{SystemTime, UNIX_EPOCH};

impl Widget{
    pub fn draw(&self, drawer:&mut Drawer){
        match self {
            Widget::Label{text, size, xy, rgba} => {
                drawer.set_font_style(*size);
                drawer.set_fill_style(*rgba);
                let (w, h) = drawer.rendered_text_wh(text);
                let (x, y) = *xy;
                drawer.draw_text(text, (x-w*0.5,y-h*0.5));
            }
            Widget::Input{focused, text, size, xy, wh, rgba} => {
                if *focused {
                    drawer.set_font_style(*size);
                    drawer.set_fill_style(*rgba);
                    let (x, y) = (xy.0-wh.0*0.5, xy.1-wh.1*0.5);
                    drawer.draw_text(text, (x, y));
                    let (w, h) = drawer.rendered_text_wh(text);
                    drawer.draw_rect((x+w, y, 1., h));
                }
                else {
                    drawer.set_font_style(*size);
                    drawer.set_fill_style(*rgba);
                    drawer.draw_text(text, *xy);
                }
            }
            _ => {}
        }
    }
    pub fn mouse_down(&mut self, xy:(f32,f32)){
        let (mx, my) = xy;
        match self {
            Widget::Input{ref mut focused, text:_, size:_, xy: (x, y), wh: (w,h), ..}=> {
                if *x<mx && mx<*x+*w && *y<my && my<*y+*h {
                    *focused = true;
                }
            }
            _ => {}
        }
    }
}
