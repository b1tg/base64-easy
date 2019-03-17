use std::{char, str};

pub fn decode(s: String) -> String {
    let s = s.trim_end_matches('=');
    let mut result = String::from("");
    for c in s.chars() {
        let maped_c = base64decode_map(c);
        result.push_str(&format!("{:06b}", maped_c));
    }
    let loop_t = result.len() / 8;
    let len = &loop_t * 8;
    let binary = &result[..len];

    let mut n = 1;
    let mut vec = Vec::new();
    while n <= loop_t {
        let slice = &binary[(8 * (n - 1))..(8 * n)];
        let intval = u8::from_str_radix(slice, 2).unwrap();
        vec.push(intval);
        n += 1;
    }
    str::from_utf8(&vec).unwrap().to_owned()
}

pub fn encode(s: String) -> String {
    let chars = s.as_bytes();
    let mut b = String::from("");
    for x in chars {
        b.push_str(&format!("{:08b}", x));
    }
    let len = b.len();
    let mut loop_t = len / 6;
    let remainder = len % 6;
    if remainder != 0 {
        b.push_str(&format!("{n:>0width$}", n = 0, width = (6 - remainder)));
        loop_t += 1;
    }
    let mut result = String::from("");
    let mut n = 1;
    while n <= loop_t {
        let slice = &b[(6 * (n - 1))..(6 * n)];
        let intval = u8::from_str_radix(slice, 2).unwrap();
        result.push_str(&base64encode_map(intval));
        n += 1;
    }

    if remainder == 2 {
        result.push_str("==");
    } else if remainder == 4 {
        result.push_str("=");
    }
    result
}

fn base64encode_map(index: u8) -> String {
    let mut offset = 0;
    if index < 26 {
        offset = 65;
    } else if index >= 26 && index < 52 {
        offset = 97 - 26;
    } else if index >= 52 && index < 62 {
        offset = 80 - 52;
    } else if index == 62 {
        return "+".to_owned();
    } else if index == 63 {
        return "/".to_owned();
    }
    let result = format!("{}", (index + offset) as char);
    result
}

fn base64decode_map(s: char) -> u8 {
    if s == '+' {
        62
    } else if s == '/' {
        63
    } else {
        let c = format!("{}", s as u8).parse::<u8>().unwrap();
        if c >= 65 && c < 91 {
            return c - 65;
        } else if c >= 97 && c < 123 {
            return c - 71;
        } else if c >= 80 && c < 90 {
            return c - 28;
        }
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_encode() {
        let input = String::from("Ma");
        let output = encode(input);
        assert_eq!("TWE=".to_string(), output);
    }

    #[test]
    fn test_decode() {
        let input = String::from("TWE");
        let output = decode(input);
        assert_eq!("Ma".to_string(), output);
    }
}
