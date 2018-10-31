extern crate common;
use common::model;
use common::protocol::{
    serialize, deserialize, Packet,
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

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws;
mod websocket;
use self::websocket::Websocket;
use self::websocket::Message;
mod text_input;
use self::text_input::TextInput;
//use self::websocket::Websocket;


extern crate quicksilver;
use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, Window, run},
    input::{Key, ButtonState, MouseButton},
};

mod scene;
use self::scene::{Game, Scene};

const WINDOW_W: i32 = 600;
const WINDOW_H: i32 = 800;
//const SERVER_ADDRESS: &'static str = "ws://aing.io:3306";
//const SERVER_ADDRESS: &'static str = "ws://127.0.0.1:3014";
const SERVER_ADDRESS: &'static str = "ws://10.64.135.197:3306";

enum State{
    GameLoading,
    Connecting,
    WaitNicknameInput,
    NicknameSending,
    InGame,
}
struct Gamedata {
    state: State,
    socket: Websocket,
    default_font: Asset<Font>,
    text_input: TextInput,
}
impl Gamedata {
    fn new() -> Gamedata{
        Gamedata{
            state: State::GameLoading,
            socket: Websocket::new(SERVER_ADDRESS),
            default_font: Asset::new(Font::load("ttf/font.ttf")),
            text_input: TextInput::new(),
        }
    }
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
impl Scene for StartMenu {
    type Data = Gamedata;
    fn update(&mut self, window: &mut Window, data: &mut Self::Data) -> Option<Box<dyn Scene<Data=Self::Data>>> {
        self.gui.update();
        let loading_spiner_widget_id = self.loading_spiner_widget_id;
        match &mut data.state {
            State::GameLoading => {
                self.active_loading_spiner();
                match data.socket.state() {
                    websocket::State::Error => {
                        Some(Box::new(Panic::new("Error: websocket fail..".to_string())))
                    }
                    websocket::State::Closed => {
                        Some(Box::new(Panic::new("Connection closed".to_string())))
                    }
                    websocket::State::Connected => {
                        data.state = State::WaitNicknameInput;
                        None
                    }
                    websocket::State::Connecting => {
                        data.state = State::GameLoading;
                        None
                    }
                    _ => {
                        None
                    }
                }
                // Done game loading
            }
            State::Connecting => {
                self.active_loading_spiner();
                None
            }
            State::WaitNicknameInput => {
                self.deactive_loading_spiner();
                let mpos = window.mouse().pos();
                self.gui.mouse_move((mpos.x, mpos.y));
                if self.is_start_button_pressed() {
                    data.state = State::NicknameSending;
                    data.socket.send(
                        Message::Binary(serialize(&Packet::Join{nickname: &self.nickname()}).unwrap()));
                }
                if window.mouse()[MouseButton::Left] == ButtonState::Pressed {
                    self.gui.mouse_down((mpos.x, mpos.y));
                }
                if window.mouse()[MouseButton::Left] == ButtonState::Released {
                    self.gui.mouse_up((mpos.x, mpos.y));
                }
                if window.keyboard()[Key::Back] == ButtonState::Pressed {
                    self.gui.key_down(naive_gui::Key::Back);
                }
                if let Some(ch) = data.text_input.char(window.keyboard()) {
                    self.gui.key_input(ch);
                }
                None
            }
            State::NicknameSending => {
                self.active_loading_spiner();
                if let Ok(msg) = data.socket.try_recv() {
                    if let Message::Binary(bytes) = msg {
                        if let Ok(decoded) = deserialize(&bytes) {
                            if let Packet::JoinResult{success} = decoded {
                                if success {
                                    data.state = State::InGame;
                                    Some(Box::new(InGame{nickname: self.nickname()}))
                                }
                                else {
                                    data.state = State::InGame;
                                    Some(Box::new(Panic::new("fail to join".to_string())))
                                }
                            } else {
                                Some(Box::new(Panic::new("fail to parse decode".to_string())))
                            }
                        } else {
                            Some(Box::new(Panic::new("fail to decode".to_string())))
                        }
                    }else {
                        Some(Box::new(Panic::new("message is not a binary type".to_string())))
                    }
                }else {
                    None
                }
            }
            _ => {
                Some(Box::new(Panic::new("invalid state".to_string())))
            }
        }
    }
    fn draw(&mut self, window: &mut Window, data: &mut Self::Data) -> Result<()> {
        match data.state {
            State::Connecting | State::NicknameSending => {
                window.clear(Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0 })?;
            }
            _ => {
                window.clear(Color::WHITE)?;
            }
        }
        let mut dc = QuickSilverDrawContext::new(window, &mut data.default_font);
        self.gui.draw(&mut dc);
        Ok(())
    }
}
struct Panic {
    gui: Gui,
}
impl Panic {
    fn new(message: String) -> Panic {
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
impl Scene for Panic{
    type Data = Gamedata;
    fn update(&mut self, window: &mut Window, data: &mut Self::Data) -> Option<Box<dyn Scene<Data=Self::Data>>> {
        None
    }
    fn draw(&mut self, window: &mut Window, data: &mut Self::Data) -> Result<()> {
        window.clear(Color::WHITE)?;
        let mut dc = QuickSilverDrawContext::new(window, &mut data.default_font);
        self.gui.draw(&mut dc);
        Ok(())
    }
}

struct InGame {
    nickname: String,
}
impl Scene for InGame {
    type Data = Gamedata;
    fn update(&mut self, window: &mut Window, data: &mut Self::Data) -> Option<Box<dyn Scene<Data=Self::Data>>> {
        None
    }
    fn draw(&mut self, window: &mut Window, data: &mut Self::Data) -> Result<()> {
        window.clear(Color::WHITE)?;
        Ok(())
    }
}

struct GameState {
    game: Game<Gamedata>,
}

impl quicksilver::lifecycle::State for GameState {
    fn new() -> Result<GameState> {
        Ok(GameState{
               game: Game{
                   current_scene: Box::new(StartMenu::new()),
                   data: Gamedata::new(),
            }})
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.game.update(window)
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.game.draw(window)
    }
}

fn main() {
    //stdweb::initialize();
    run::<GameState>("Gradual Suicide", Vector::new(WINDOW_W, WINDOW_H), Settings::default());
}
