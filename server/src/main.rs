extern crate common;
use common::model;
use common::protocol::{
   serialize, deserialize, Packet, 
};
extern crate ws;

use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};
use ws::util::Token;

#[derive(Copy, Clone)]
struct User {
    cid: Option<model::Charactor_id>,
}
struct Token2User {
    vec: Vec<User>,
}
impl Token2User {
    fn put(&mut self, token: &Token, user: User) {
        if self.vec.len() >= token.0 {
            self.vec.resize((token.0+1)*2, User{cid:None});
        }
        self.vec[token.0] = user;
    }
    fn get(&mut self, token: &Token) -> &User {
        &self.vec[token.0]
    }
    fn remove(&mut self, token: &Token) {
        self.vec[token.0] = User{cid:None};
    }
    fn new() -> Token2User{ Token2User{ vec: Vec::new() } }
}
struct Server { 
    socket: ws::Sender, 
    count: u32,
    users: Token2User,
}
impl Server {
    fn new(socket: ws::Sender) -> Server {
      Server{
          socket: socket, 
          count: 0,
          users: Token2User::new(),
      }
    }
}
impl ws::Handler for Server {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("user {:?} coming", self.socket.token());
        self.count += 1;
        self.users.put(&self.socket.token(), User{cid: Some(self.socket.token().0)});
        Ok(())
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("token: {:?}", self.users.get(&self.socket.token()).cid);
        let user = self.users.get(&self.socket.token());
        if let ws::Message::Binary(bytes) = msg {
            if let Ok(decoded) = deserialize(&bytes) {
                match decoded {
                    Packet::Join {nickname} => {
                        println!("Join msg {}", nickname);
                        let res = serialize(&Packet::JoinResult{success:true});
                        if let Ok(encoded) = res {
                            self.socket.send(encoded)
                        }
                        else {
                            println!("fail to encoding msg");
                            Ok(())
                        }
                    }
                    _ => { Ok(()) }
                }
            } else {
                println!("fail to deserialize msg");
                Ok(())
            }
        }  else {
            Ok(())
        }
    }
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        match code {
            ws::CloseCode::Normal => println!("The client is done with the connection."),
            ws::CloseCode::Away   => println!("The client is leaving the site."),
            ws::CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
        self.count -= 1
    }
    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

fn main() {
  ws::listen("127.0.0.1:3012", |socket| {
      Server::new(socket)
  }).unwrap()
} 
