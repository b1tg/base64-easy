# base64-easy
base64 implement in Rust.

[Documentation](https://docs.rs/base64-easy)

## Usage

use as lib

```rust
extern crate base64_easy;
use base64_easy::{encode, decode};

let a = encode("abc".to_string());

let b = decode("YWJj".to_string());

```


use as binary
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
