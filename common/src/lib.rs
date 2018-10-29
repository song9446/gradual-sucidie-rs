#[macro_use]
extern crate serde_derive;
extern crate bincode;
pub mod model;
pub mod protocol;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = super::protocol::Packet::Join{nickname: "hi"};
        let encoded: Vec<u8> = super::protocol::serialize(&x).unwrap();
        let decoded: super::protocol::Packet = super::protocol::deserialize(&encoded).unwrap();
        assert_eq!(x, decoded);
    }
}
