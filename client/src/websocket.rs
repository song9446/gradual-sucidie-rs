// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};

#[cfg(not(target_arch = "wasm32"))]
use std::sync::{
    Arc,
    Mutex,
};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{
    channel,
    Sender,
    Receiver,
    RecvError,
    TryRecvError,
};

#[cfg(target_arch = "wasm32")]
use std::{
    cell::{RefCell, Cell},
    rc::Rc,
};


#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Connecting,
    Connected,
    Closed,
    Error,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}
#[cfg(target_arch = "wasm32")]
pub use stdweb;
#[cfg(target_arch = "wasm32")]
pub use stdweb::{
    web::IEventTarget,
    traits::IMessageEvent,
};

#[cfg(not(target_arch = "wasm32"))]
pub use ws::Message;
//pub type Message = ws::Message;

#[cfg(target_arch="wasm32")]
pub struct Websocket{
    socket: Option<stdweb::web::WebSocket>,
    receiver: Rc<RefCell<Option<Message>>>,
    state: Rc<Cell<State>>,
}
#[cfg(target_arch="wasm32")]
impl Websocket {
    pub fn new(address:&'static str) -> Websocket {
        let receiver: Rc<RefCell<Option<Message>>> = Rc::new(RefCell::new(None));
        let state = Rc::new(Cell::new(State::Connecting));
        let ws_result = stdweb::web::WebSocket::new(address);
        match ws_result {
            Ok(ws) => {
                ws.set_binary_type(stdweb::web::SocketBinaryType::ArrayBuffer);
                let state_cloned = state.clone();
                ws.add_event_listener(move |_: stdweb::web::event::SocketOpenEvent| {
                    state_cloned.set(State::Connected);
                });
                let state_cloned = state.clone();
                ws.add_event_listener(move |error: stdweb::web::event::SocketErrorEvent| {
                    state_cloned.set(State::Error);
                });
                let state_cloned = state.clone();
                ws.add_event_listener(move |event: stdweb::web::event::SocketCloseEvent| {
                    if state_cloned.get() != State::Error {
                        state_cloned.set(State::Closed);
                    }
                });
                let receiver_cloned = receiver.clone();
                let state_cloned = state.clone();
                ws.add_event_listener(move |event: stdweb::web::event::SocketMessageEvent| {
                    let msg = 
                        if let Some(bytes) = event.data().into_array_buffer(){
                            let bytes: Vec<u8> = bytes.into();
                            Message::Binary(bytes)
                        } else {
                            let text = event.data().into_text().unwrap();
                            Message::Text(text)
                        };
                    *receiver_cloned.borrow_mut() = Some(msg);
                });
                Websocket{
                    socket: Some(ws),
                    receiver: receiver,
                    state: state,
                }
            }
            Err(error) => {
                state.set(State::Error);
                Websocket{
                    socket: None,
                    receiver: receiver,
                    state: state,
                }
            }
        }
    }
    pub fn state(&mut self) -> State{
        self.state.get()
    }
    pub fn send(&mut self, message: Message) -> Result<(), ()> {
        if let Some(ref mut sender) = self.socket {
            match message {
                Message::Binary(bytes) => sender.send_bytes(&bytes).or(Err(())),
                Message::Text(text) => sender.send_text(&text).or(Err(())),
            }
        }
        else {
            Err(())
        }
    }
    pub fn recv(&self) -> Result<Message, ()> {
        while self.receiver.borrow().is_none() { }
        let msg = self.receiver.borrow().clone().unwrap();
        *self.receiver.borrow_mut() = None;
        Ok(msg)
    }
    pub fn try_recv(&self) -> Result<Message, ()> {
        if self.receiver.borrow().is_none() {
            Err(())
        } else{
            let msg = self.receiver.borrow().clone().unwrap();
            *self.receiver.borrow_mut() = None;
            Ok(msg)
        }
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
                *state_cloned.lock().unwrap() = State::Error;
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
        *self.state.lock().unwrap() = State::Error;
    }
}
