#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[cfg(not(target_arch = "wasm32"))]
extern crate ws;

// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};

use std::sync::{
    Arc,
    Mutex,
};
use std::sync::mpsc::{
    channel,
    Sender,
    Receiver,
    RecvError,
    TryRecvError,
};


#[derive(Clone)]
pub enum State {
    Connecting,
    Connected,
    Closed,
    Error(String),
}

#[cfg(target_arch = "wasm32")]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}
#[cfg(not(target_arch = "wasm32"))]
pub use ws::Message;
//pub type Message = ws::Message;

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
        let (tx, rx) = channel();
        let res = stdweb::web::WebSocket::new(address);
        if let Ok(socket) = res {
            Websocket{
                socket: stdweb::web::WebSocket::new(address),
                receiver: rx,
                sender: tx,
            }
        }
        else {
            Err(())
        }
    }
    pub fn recv(&self) -> Result<Message, RecvError> {
        self.receiver.recv()
    }
    pub fn try_recv(&self) -> Result<Message, TryRecvError> {
        self.receiver.try_recv()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Websocket{
    thread: std::thread::JoinHandle<()>,
    receiver: Receiver<Message>,
    sender: Arc<Mutex<Option<ws::Sender>>>,
    state: Arc<Mutex<State>>,
}
#[cfg(not(target_arch = "wasm32"))]
impl Websocket {
    pub fn new(address:&'static str) -> Websocket {
        let (receiver_tx, receiver_rx) = channel();
        let state = Arc::new(Mutex::new(State::Connecting));
        let state_cloned = state.clone();
        let sender: Arc<Mutex<Option<ws::Sender>>> = Arc::new(Mutex::new(None));
        let sender_cloned = sender.clone();
        //let tx = tx.clone();
        let thread = std::thread::spawn(move || {
            if let Err(error) = ws::connect(address, |out| {
                *sender_cloned.lock().unwrap() = Some(out);
                Client {
                    tx: receiver_tx.clone(),
                    state: state_cloned.clone(),
                }
            }) {
                *state_cloned.lock().unwrap() = State::Error(format!("{:?}", error));
                println!("{:?}", error);
            }
        });
        Websocket{
            thread: thread,
            receiver: receiver_rx,
            sender: sender,
            state: state,
        }
    }
    pub fn state(&mut self) -> State{
        self.state.lock().unwrap().clone()
    }
    pub fn send(&mut self, message: Message) -> Result<(), ()> {
        if let Some(ref sender) = *self.sender.lock().unwrap() {
            sender.send(message).or(Err(()))
        }
        else {
            Err(())
        }
    }
    pub fn recv(&self) -> Result<Message, RecvError> {
        self.receiver.recv()
    }
    pub fn try_recv(&self) -> Result<Message, TryRecvError> {
        self.receiver.try_recv()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Websocket {
    fn drop(&mut self) {
        if let Some(ref sender) = *self.sender.lock().unwrap(){
            sender.close(ws::CloseCode::Normal);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct Client{
    tx: Sender<Message>,
    state: Arc<Mutex<State>>,
}
#[cfg(not(target_arch = "wasm32"))]
impl ws::Handler for Client {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        *self.state.lock().unwrap() = State::Connected;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        self.tx.send(msg);
        println!("some thing is comming");
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        *self.state.lock().unwrap() = State::Closed;
    }

    fn on_error(&mut self, err: ws::Error) {
        *self.state.lock().unwrap() = State::Error(format!("{:?}", err));
    }
}
