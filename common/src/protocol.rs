#[macro_use]

pub use bincode::{serialize, deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Packet<'a> {
    Join{nickname: &'a str, },
    JoinResult {success: bool,},
    Chat { msg: &'a str, },
}

