#![feature(proc_macro)]
extern crate proc_macro;
extern crate regex;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(AnswerFn)]
pub fn derive_packet_enum(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let mut i = 0;
    loop {
        input_string.
    }
    let s:String = input_string.chars()
        .map(|x| match x {
            '!' => '?',
            'A'...'Z' => 'X',
            'a'...'z' => 'x',
            _ => x}
            ).collect();
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
    let in_decimal_system = i32::from_str_radix(&input_string, 2).unwrap();
    //convert back to string
    let output_string = in_decimal_system.to_string();
    //convert back to token stream
    to_tokenstream(&output_string)
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        enum p {
            struct a{x:i32}
            struct b(i32, f32)
        }
        assert_eq!(2 + 2, 4);
    }
}
