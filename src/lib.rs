//! Usage:
//! ```
//! extern crate base64_easy;
//! use base64_easy::{encode, decode};
//! 
//! let a = encode("abc".to_string());
//! 
//! let b = decode("YWJj".to_string());
//! ```
//! 


mod base64;

pub use self::base64::{decode, encode};
