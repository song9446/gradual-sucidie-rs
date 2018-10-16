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
                drawer.draw_text(text, *xy);
            }
            Widget::Input{focused, text, size, xy, wh, rgba} => {
                if *focused {
                    drawer.set_font_style(*size);
                    drawer.set_fill_style(*rgba);
                    drawer.draw_text(text, *xy);
                    let (w, h) = drawer.rendered_text_wh(text);
                    drawer.draw_rect((*xy.0+w, *xy.1, 1, h));
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
                if x<mx && mx<x+w && y<my && my<y+h {
                    *focused = true;
                }
            }
            _ => {}
        }
    }
}
