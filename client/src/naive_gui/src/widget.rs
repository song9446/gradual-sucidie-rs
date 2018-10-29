#[macro_use]
#[derive(Clone, Copy)]
pub enum Key {
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    F13, F14, F15, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right,
    Down, Back, Return, Space, Compose, Caret, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5,
    Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator,
    Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl,
    LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward,
    NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause,
    Power, PrevTrack, RAlt, RBracket, RControl, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract,
    Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome,
    WebRefresh, WebSearch, WebStop, Yen,
}
pub enum Widget {
    #[derive(Default)]
    Label{
        text:String, 
        size:f32,
        xy:(f32, f32),  
        rgba:(f32, f32, f32, f32),
    },
    #[derive(Default)]
    Input{
        focused: bool,
        text:String, 
        size:f32,
        xy:(f32, f32),  
        wh:(f32, f32),
        rgba:(f32, f32, f32, f32),
    },
    Button{
        hovered: bool,
        pressed: bool,
        text:String, 
        size:f32,
        xy:(f32, f32),
        wh:(f32, f32),
        rgba:(f32, f32, f32, f32),
    },
    LoadingSpinner{
        active:bool,
        xy:(f32, f32),
        radius:f32,
        angle:f32,
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
                let (x, y) = (xy.0-wh.0*0.5, xy.1-wh.1*0.5);
                drawer.set_font_style(*size);
                drawer.set_fill_style(*rgba);
                drawer.draw_rect((x, y+wh.1, wh.0, 2.));
                if *focused {
                    drawer.draw_text(text, (x, y));
                    let (w, h) = drawer.rendered_text_wh(text);
                    drawer.draw_rect((x+w, y, 1., h));
                }
                else {
                    drawer.draw_text(text, (x, y));
                }
            }
            Widget::Button{hovered, pressed, text, size, xy, wh, rgba} => {
                let (x, y) = (xy.0-wh.0*0.5, xy.1-wh.1*0.5);
                drawer.set_font_style(*size);
                drawer.set_fill_style(*rgba);
                let (w, h) = drawer.rendered_text_wh(text);
                if *pressed {
                    drawer.draw_rect((x, y, wh.0, 1.));
                    drawer.draw_rect((x, y, 1., wh.1));
                    drawer.draw_rect((x, y+wh.1, wh.0, 1.));
                    drawer.draw_rect((x+wh.0, y, 1., wh.1));
                    let (x, y) = (x+(wh.0-w)*0.5, y+(wh.1-h)*0.5);
                    drawer.draw_text(text, (x, y));
                }
                else {
                    let (x, y) = (xy.0-wh.0*0.5-2., xy.1-wh.1*0.5-2.);
                    drawer.draw_rect((x, y, wh.0, 1.));
                    drawer.draw_rect((x, y, 1., wh.1));
                    drawer.draw_rect((x, y+wh.1, wh.0, 1.));
                    drawer.draw_rect((x+wh.0, y, 1., wh.1));
                    drawer.draw_rect((x+3., y+wh.1, wh.0, 3.));
                    drawer.draw_rect((x+wh.0, y+3., 3., wh.1));
                    let (x, y) = (x+(wh.0-w)*0.5, y+(wh.1-h)*0.5);
                    drawer.draw_text(text, (x, y));
                }
            }
            Widget::LoadingSpinner{active, xy, radius, angle, rgba} => {
                if !*active { return; }
                const MAX_ANGLE:f32 = std::f32::consts::PI*1.;
                let (x, y) = (xy.0, xy.1);
                let (cx, cy) = (x+angle.sin()**radius, y-angle.cos()**radius);
                drawer.draw_rect((cx, cy, 3., 3.));
            }
            _ => {}
        }
    }
    pub fn update(&mut self){
        match self {
            Widget::LoadingSpinner{active, angle, ..} => {
                if !*active { return; }
                const MAX_ANGLE:f32 = std::f32::consts::PI*100.;
                *angle += (*angle).min(MAX_ANGLE-*angle).max(0.1)*0.08;
                if *angle > MAX_ANGLE { *angle -= MAX_ANGLE; }
            }
            _ => {
            }
        }
    }
    pub fn mouse_move(&mut self, xy:(f32,f32)){
        let (mx, my) = xy;
        match self {
            Widget::Button{ref mut hovered, pressed, xy: (x, y), wh: (w,h), ..}=> {
                if *x-(*w)*0.5<mx && mx<*x+(*w)*0.5 && *y-(*h)*0.5<my && my<*y+(*h)*0.5 {
                    *hovered = true;
                }
                else {
                    *hovered = false;
                    *pressed = false;
                }
            }
            _ => {}
        }
    }
    pub fn mouse_down(&mut self, xy:(f32,f32)){
        let (mx, my) = xy;
        match self {
            Widget::Input{ref mut focused, text:_, size:_, xy: (x, y), wh: (w,h), ..}=> {
                if *x-(*w)*0.5<mx && mx<*x+(*w)*0.5 && *y-(*h)*0.5<my && my<*y+(*h)*0.5 {
                    *focused = true;
                }
                else {
                    *focused = false;
                }
            }
            Widget::Button{ref mut hovered, pressed, xy: (x, y), wh: (w,h), ..}=> {
                if *x-(*w)*0.5<mx && mx<*x+(*w)*0.5 && *y-(*h)*0.5<my && my<*y+(*h)*0.5 {
                    *pressed = true;
                }
            }
            _ => {}
        }
    }
    pub fn mouse_up(&mut self, xy:(f32,f32)){
        let (mx, my) = xy;
        match self {
            Widget::Button{ref mut hovered, pressed, xy: (x, y), wh: (w,h), ..}=> {
                if *x-(*w)*0.5<mx && mx<*x+(*w)*0.5 && *y-(*h)*0.5<my && my<*y+(*h)*0.5 {
                    *pressed = false;
                }
            }
            _ => {}
        }
    }
    pub fn key_down(&mut self, key: Key) {
        match key {
            Key::Back => {
                match self { 
                    Widget::Input{ref mut focused, text, size:_, xy: (x, y), wh: (w,h), ..}=> {
                        if *focused {
                            text.pop();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    pub fn key_input(&mut self, ch: char) {
        match self { 
            Widget::Input{ref mut focused, text, size:_, xy: (x, y), wh: (w,h), ..}=> {
                if *focused {
                    text.push(ch);
                }
            }
            _ => {}
        }
    }
}

