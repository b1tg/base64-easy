# base64-easy
base64 implement in Rust.

# Usage

## As lib

```rust
extern crate base64_easy;
use base64_easy::{encode, decode};

let a = encode("abc".to_string());

let b = decode("YWJj".to_string());

```


## As binary
```

# build
cargo build -- example simple

# encode   
$ ./simple e abc77                                                                                                    
YWJjNzc=

# decode   
$ ./simple d YWJjNzc=                                                                                           
abc77
```
