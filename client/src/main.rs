#[macro_use]
extern crate stdweb;
extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, Window, run},
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

const WINDOW_W: i32 = 300;
const WINDOW_H: i32 = 400;
const SERVER_ADDRESS: &'static str = "http://aing.io:30";

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
    socket: stdweb::web::WebSocket,
    default_font: Asset<Font>,
}

impl quicksilver::lifecycle::State for Game {
    fn new() -> Result<Game> {
        Ok(Game{
            state: State::StartMenu(StartMenu::new()),
            socket: stdweb::web::WebSocket::new("").unwrap(),
            default_font: Asset::new(Font::load("ttf/font.ttf")),
        })
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
    stdweb::initialize();
    run::<Game>("Gradual Suicide", Vector::new(WINDOW_W, WINDOW_H), Settings::default());
}
