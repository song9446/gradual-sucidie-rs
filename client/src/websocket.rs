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
enum Message {
    Bytes(Vec<u8>),
    Str(String),
}

pub struct Websocket{
    #[cfg(target_arch="wasm32")]
    socket: stdweb::web::WebSocket,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}
impl Websocket {
    pub fn new(address:&'static str) -> Websocket {
        let (s, r) = channel();
        Websocket{
            #[cfg(target_arch="wasm32")]
            socket: stdweb::web::WebSocket::new(address),
            receiver: r,
            sender: s,
        }
    }
    pub fn recv(&self) -> Result<Message, RecvError> {
        self.receiver.recv()
    }
}

