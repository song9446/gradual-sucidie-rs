#[macro_use]
extern crate stdweb;
extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, Window, run},
    input::{Key, ButtonState, MouseButton},
};

extern crate naive_gui;
use naive_gui::{
    Gui,
    Widget,
    Drawer,
};
mod gui;
use self::gui::QuickSilverDrawContext;
use std::cmp;

mod websocket;
use self::websocket::Websocket;

const WINDOW_W: i32 = 300;
const WINDOW_H: i32 = 400;
const SERVER_ADDRESS: &'static str = "http://aing.io:30";

const KEY_LIST: &[Key] = &[Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9, Key::Key0, Key::A, Key::B, Key::C, Key::D,
    Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L, Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X, Key::Y, Key::Z,
    Key::Escape, Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12,
    Key::F13, Key::F14, Key::F15, Key::Snapshot, Key::Scroll, Key::Pause, Key::Insert, Key::Home, Key::Delete, Key::End, Key::PageDown, Key::PageUp, Key::Left, Key::Up, Key::Right,
    Key::Down, Key::Back, Key::Return, Key::Space, Key::Compose, Key::Caret, Key::Numlock, Key::Numpad0, Key::Numpad1, Key::Numpad2, Key::Numpad3, Key::Numpad4, Key::Numpad5,
    Key::Numpad6, Key::Numpad7, Key::Numpad8, Key::Numpad9, Key::AbntC1, Key::AbntC2, Key::Add, Key::Apostrophe, Key::Apps, Key::At, Key::Ax, Key::Backslash, Key::Calculator,
    Key::Capital, Key::Colon, Key::Comma, Key::Convert, Key::Decimal, Key::Divide, Key::Equals, Key::Grave, Key::Kana, Key::Kanji, Key::LAlt, Key::LBracket, Key::LControl,
    Key::LShift, Key::LWin, Key::Mail, Key::MediaSelect, Key::MediaStop, Key::Minus, Key::Multiply, Key::Mute, Key::MyComputer, Key::NavigateForward,
    Key::NavigateBackward, Key::NextTrack, Key::NoConvert, Key::NumpadComma, Key::NumpadEnter, Key::NumpadEquals, Key::OEM102, Key::Period, Key::PlayPause,
    Key::Power, Key::PrevTrack, Key::RAlt, Key::RBracket, Key::RControl, Key::RShift, Key::RWin, Key::Semicolon, Key::Slash, Key::Sleep, Key::Stop, Key::Subtract,
    Key::Sysrq, Key::Tab, Key::Underline, Key::Unlabeled, Key::VolumeDown, Key::VolumeUp, Key::Wake, Key::WebBack, Key::WebFavorites, Key::WebForward, Key::WebHome,
    Key::WebRefresh, Key::WebSearch, Key::WebStop, Key::Yen];

struct StartMenu {
    gui: Gui,
    nickname_widget_id: usize,
}
impl StartMenu {
    fn new() -> StartMenu{
        let mut gui = Gui::new();
        const TITLE_FONT_SIZE: f32 = (WINDOW_W as f32)*0.1;
        const NICKNAME_INPUT_FONT_SIZE: f32 = (WINDOW_W as f32)*0.08;
        const BLACK: (f32, f32, f32, f32) = (1., 1., 1., 0.);
        gui.put(
            Widget::Label{
                text:"Gradual Suicide".to_string(), 
                size:TITLE_FONT_SIZE, 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.3), 
                rgba:BLACK});
        gui.put(
            Widget::Label{
                text:"Nickname?".to_string(), 
                size:NICKNAME_INPUT_FONT_SIZE, 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.5), 
                rgba:BLACK});
        let nickname_widget_id = gui.put(
            Widget::Input{
                size:NICKNAME_INPUT_FONT_SIZE, 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.6),
                wh:((WINDOW_W as f32)*0.5, NICKNAME_INPUT_FONT_SIZE), 
                rgba:BLACK, 
                text: "".to_string(),  
                focused: true, });
        StartMenu {
            gui: gui,
            nickname_widget_id: nickname_widget_id, }
    }
    fn nickname(&mut self) -> String {
        if let Widget::Label{text:text, ..} = self.gui.get(self.nickname_widget_id) {
             text.to_string()
        } else{
            "".to_string()
        }
    }
}
struct Panic {
    gui: Gui,
}
impl Panic {
    fn new(message:&'static str) -> Panic {
        let mut gui = Gui::new();
        const RED: (f32, f32, f32, f32) = (1., 1., 1., 0.);
        gui.put(
            Widget::Label{
                text:message.to_string(),
                size:12., 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.5), 
                rgba:RED});
        Panic {
            gui: gui,
        }
    }
}

enum State {
    StartMenu(StartMenu),
    Panic(Panic),
    InGame,
}

struct Game {
    state: State,
//    socket: stdweb::web::WebSocket,
    default_font: Asset<Font>,
    keys_ready: [bool; 256],
}

impl quicksilver::lifecycle::State for Game {
    fn new() -> Result<Game> {
        Ok(Game{
            state: State::StartMenu(StartMenu::new()),
//            socket: stdweb::web::WebSocket::new("").unwrap(),
            default_font: Asset::new(Font::load("ttf/font.ttf")),
            keys_ready: [false; 256],
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match &mut self.state {
            State::Panic(Panic{ref mut gui}) => {
            }
            State::StartMenu(ref mut start_menu) => {
                if window.mouse()[MouseButton::Left] == ButtonState::Pressed {
                    let mpos = window.mouse().pos();
                    start_menu.gui.mouse_down((mpos.x, mpos.y));
                }
                if window.keyboard()[Key::Back].is_down(){
                    if self.keys_ready[Key::Back as usize] {
                        self.keys_ready[Key::Back as usize] = false;
                        start_menu.gui.key_down(naive_gui::Key::Back);
                    }
                }
                else {
                    self.keys_ready[Key::Back as usize] = true;
                }
                for i in (Key::Key1 as u8)..(Key::Z as u8 + 1) {
                    if window.keyboard()[KEY_LIST[i as usize]].is_down(){
                        if self.keys_ready[i as usize] {
                            self.keys_ready[i as usize] = false;
                            let ch = 
                                if i >= (Key::A as u8) {
                                    (('a' as u8) + (i - (Key::A as u8))) as char
                                }else if i == (Key::Key0 as u8) {
                                    '0'
                                }else {
                                    (('1' as u8) + (i - (Key::Key1 as u8))) as char
                                };
                            start_menu.gui.key_input(ch);
                        }
                    }
                    else{
                        self.keys_ready[i as usize] = true;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match &mut self.state {
            State::Panic(Panic{ref mut gui}) => {
                window.clear(Color::WHITE)?;
                let mut dc = QuickSilverDrawContext::new(window, &mut self.default_font);
                gui.draw(&mut dc);
            }
            State::StartMenu(ref mut start_menu) => {
                window.clear(Color::WHITE)?;
                let mut dc = QuickSilverDrawContext::new(window, &mut self.default_font);
                start_menu.gui.draw(&mut dc);
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() {
    //stdweb::initialize();
    run::<Game>("Gradual Suicide", Vector::new(WINDOW_W, WINDOW_H), Settings::default());
}
