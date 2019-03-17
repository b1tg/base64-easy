extern crate base64_easy;

use std::env;
use base64_easy::{decode, encode};
fn main() {
    let option = match env::args().nth(1) {
        Some(o) => o,
        None => {
            println!("Usage: base64-simple [d | e] <payload>");
            return;
        }
    };
    let payload = match env::args().nth(2) {
        Some(p) => p,
        None => {
            println!("Usage: base64-simple [d | e] <payload>");
            return;
        }
    };

    if option == "e" {
        println!("{}", encode(payload));
    } else if option == "d" {
        println!("{}", decode(payload).unwrap());
    }
}
