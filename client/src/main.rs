#[macro_use]
extern crate common;
use common::model;
use common::protocol::{
    serialize, deserialize, Packet,
};
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
use self::websocket::Message;
mod text_input;
use self::text_input::TextInput;
//use self::websocket::Websocket;

const WINDOW_W: i32 = 300;
const WINDOW_H: i32 = 400;
const SERVER_ADDRESS: &'static str = "ws://127.0.0.1:3012";

enum State{
    GameLoading,
    Connecting,
    WaitNicknameInput,
    NicknameSending,
    InGame,
}
struct StartMenu {
    gui: Gui,
    loading_spiner_widget_id: usize,
    game_start_button_id: usize,
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
        let game_start_button_id = gui.put(
            Widget::Button{
                hovered: false,
                pressed: false,
                text:"Start!".to_string(), 
                size:NICKNAME_INPUT_FONT_SIZE, 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.7),
                wh:((WINDOW_W as f32)*0.5, NICKNAME_INPUT_FONT_SIZE), 
                rgba:BLACK});
        let loading_spiner_widget_id = gui.put(
            Widget::LoadingSpinner{
                active: false,
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.5),
                angle: 0.,
                radius: 30.,
                rgba:BLACK});
        StartMenu {
            gui,
            nickname_widget_id, 
            loading_spiner_widget_id,
            game_start_button_id,
        }
    }
    fn nickname(&mut self) -> String {
        if let Widget::Label{text, ..} = self.gui.get(self.nickname_widget_id) {
            text.to_string()
        } else{
            "".to_string()
        }
    }
    fn active_loading_spiner(&mut self) {
        if let Widget::LoadingSpinner{ref mut active, ..} = self.gui.get_mut(self.loading_spiner_widget_id) {
            *active = true;
        }
    }
    fn deactive_loading_spiner(&mut self) {
        if let Widget::LoadingSpinner{ref mut active, ..} = self.gui.get_mut(self.loading_spiner_widget_id) {
            *active = false;
        }
    }
    fn is_start_button_pressed(&mut self) -> bool {
        if let Widget::Button{pressed, ..} = self.gui.get_mut(self.game_start_button_id) {
            *pressed
        } else {
            false
        }
    }
}
struct Panic {
    gui: Gui,
}
impl Panic {
    fn new(message:String) -> Panic {
        let mut gui = Gui::new();
        const RED: (f32, f32, f32, f32) = (1., 1., 1., 0.);
        gui.put(
            Widget::Label{
                text:message,
                size:12., 
                xy:((WINDOW_W as f32)*0.5, (WINDOW_H as f32)*0.5), 
                rgba:RED});
        Panic {
            gui: gui,
        }
    }
}

struct InGame {
    nickname: String,
}

enum Scene {
    StartMenu(StartMenu),
    Panic(Panic),
    InGame(InGame),
}

struct Game {
    state: State,
    scene: Scene,
    socket: Websocket,
    default_font: Asset<Font>,
    text_input: TextInput,
}

impl quicksilver::lifecycle::State for Game {
    fn new() -> Result<Game> {
        Ok(Game{
            state: State::GameLoading,
            scene: Scene::StartMenu(StartMenu::new()),
            socket: Websocket::new(SERVER_ADDRESS),
            default_font: Asset::new(Font::load("ttf/font.ttf")),
            text_input: TextInput::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match &mut self.scene {
            Scene::Panic(Panic{ref mut gui}) => {
            }
            Scene::StartMenu(ref mut start_menu) => {
                start_menu.gui.update();
                let loading_spiner_widget_id = start_menu.loading_spiner_widget_id;
                match &mut self.state {
                    State::GameLoading => {
                        start_menu.active_loading_spiner();
                        match self.socket.state() {
                            websocket::State::Error(ref msg) => {
                                self.scene = Scene::Panic(Panic::new((*msg).clone()));
                                return Ok(());
                            }
                            websocket::State::Closed => {
                                self.scene = Scene::Panic(Panic::new("Error: Connect Server Fail".to_string()));
                                return Ok(());
                            }
                            websocket::State::Connected => {
                                self.state = State::WaitNicknameInput;
                            }
                            websocket::State::Connecting => {
                                self.state = State::Connecting;
                            }
                            _ => {}
                        }
                        // Done game loading
                    }
                    State::Connecting => {
                        start_menu.active_loading_spiner();
                    }
                    State::WaitNicknameInput => {
                        start_menu.deactive_loading_spiner();
                        let mpos = window.mouse().pos();
                        start_menu.gui.mouse_move((mpos.x, mpos.y));
                        if start_menu.is_start_button_pressed() {
                            self.state = State::NicknameSending;
                            self.socket.send(
                                Message::Binary(serialize(&Packet::Join{nickname: &start_menu.nickname()}).unwrap()));
                        }
                        if window.mouse()[MouseButton::Left] == ButtonState::Pressed {
                            start_menu.gui.mouse_down((mpos.x, mpos.y));
                        }
                        if window.mouse()[MouseButton::Left] == ButtonState::Released {
                            start_menu.gui.mouse_up((mpos.x, mpos.y));
                        }
                        if window.keyboard()[Key::Back] == ButtonState::Pressed {
                            start_menu.gui.key_down(naive_gui::Key::Back);
                        }
                        if let Some(ch) = self.text_input.char(window.keyboard()) {
                            start_menu.gui.key_input(ch);
                        }
                    }
                    State::NicknameSending => {
                        start_menu.active_loading_spiner();
                        if let Ok(msg) = self.socket.try_recv() {
                            if let Message::Binary(bytes) = msg {
                                if let Ok(decoded) = deserialize(&bytes) {
                                    if let Packet::JoinResult{success} = decoded {
                                        if success {
                                            self.scene = Scene::InGame(InGame{nickname: start_menu.nickname()});
                                            self.state = State::InGame;
                                            println!("??");
                                        }
                                        else {
                                            self.scene = Scene::Panic(Panic::new("fail to join".to_string()));
                                            self.state = State::InGame;
                                            println!("!!");
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        self.scene = Scene::Panic(Panic::new("invalid state".to_string()));
                        return Ok(());
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match &mut self.scene {
            Scene::Panic(Panic{ref mut gui}) => {
                window.clear(Color::WHITE)?;
                let mut dc = QuickSilverDrawContext::new(window, &mut self.default_font);
                gui.draw(&mut dc);
            }
            Scene::StartMenu(ref mut start_menu) => {
                match self.state {
                    State::Connecting | State::NicknameSending => {
                        window.clear(Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0 })?;
                    }
                    _ => {
                        window.clear(Color::WHITE)?;
                    }
                }
                let mut dc = QuickSilverDrawContext::new(window, &mut self.default_font);
                start_menu.gui.draw(&mut dc);
            }
            Scene::InGame(ref mut in_game) => {
                window.clear(Color::WHITE)?;
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
