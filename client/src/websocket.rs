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
#[cfg(target_arch = "wasm32")]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

#[cfg(not(target_arch = "wasm32"))]
pub type Message = ws::Message;

pub struct Websocket{
    #[cfg(target_arch="wasm32")]
    socket: stdweb::web::WebSocket,
    #[cfg(not(target_arch = "wasm32"))]
    thread: std::thread::Thread,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}
impl Websocket {
    #[cfg(target_arch="wasm32")]
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
    #[cfg(target_arch="wasm32")]
    pub fn new(address:&'static str) -> Result<Websocket, &'static str> {
        let (s, r) = channel();
        let thread = std::thread::spawn(move || {
            ws::connect(address, |out| {
                move |msg| {
                    s.send(msg)
                }
            });
        });
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

impl Drop for Websocket {
    fn drop(&mut self) {
    }
}
