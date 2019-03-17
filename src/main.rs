use std::env;
mod base64;
use self::base64::{decode, encode};
fn main() {
    let option = match env::args().nth(1) {
        Some(o) => o,
        None => {
            println!("Usage: c-base64 [d | e] <payload>");
            return;
        }
    };
    let payload = match env::args().nth(2) {
        Some(p) => p,
        None => {
            println!("Usage: c-base64 [d | e] <payload>");
            return;
        }
    };

    if option == "e" {
        println!("{}", encode(payload));
    } else if option == "d" {
        println!("{}", decode(payload).unwrap());
    }
}
