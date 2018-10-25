#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws

// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};

use std::sync::mpsc::{
    channel,
    Sender,
    Receiver,
    RecvError,
};
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

pub enum State {
    Connected,
    Done,
    Error(&'static str),
}

#[cfg(not(target_arch = "wasm32"))]
pub type Message = ws::Message;

#[cfg(target_arch="wasm32")]
pub struct Websocket{
    socket: stdweb::web::WebSocket,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
    state: Arc<Mutex<State>>,
}
#[cfg(target_arch="wasm32")]
impl Websocket {
    pub fn new(address:&'static str) -> Result<Websocket, &'static str> {
        let (s, r) = channel();
        let res = stdweb::web::WebSocket::new(address);
        if let Ok(socket) = res {
            Websocket{
                socket: stdweb::web::WebSocket::new(address),
                receiver: r,
                sender: s,
            }
        }
        else {
            Err(())
        }
    }
    pub fn recv(&self) -> Result<Message, RecvError> {
        self.receiver.recv()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Websocket{
    thread: std::thread::Thread,
    receiver: Receiver<Message>,
    sender: Arc<Mutex<Option<ws::Sender<Message>>>>,
    state: Arc<Mutex<State>>,
}
#[cfg(not(target_arch = "wasm32"))]
impl Websocket {
    pub fn new(address:&'static str) -> Result<Websocket, &'static str> {
        let (s, r) = channel();
        let sender: Arc<Mutex<Option<Sender<Message>>>> = Arc::new(Mutex::new(None));
        let sender2 = sender.clone();
        let thread = std::thread::spawn(move || {
            ws::connect(address, |out| {
                *sender2.lock().unwrap() = Some(out);
                move |msg| {
                    s.send(msg)
                }
            });
        });
        Websocket{
            socket: stdweb::web::WebSocket::new(address),
            receiver: r,
            sender: s,
        }
    }
    pub fn state() -> Result<(), &'static str>{
    }
}

impl Drop for Websocket {
    fn drop(&mut self) {
    }
}
