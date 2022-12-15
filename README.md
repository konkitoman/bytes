# Bytes

[![Crates.io](https://img.shields.io/crates/v/bytes-kman.svg)](https://crates.io/crates/bytes-kman)

## is used for serialize and deserialize

    is useful because is more faster and smaller then json!
    But not human readable!

## Examples

```rust
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct Package{
    id: u16,
    message: String
}


pub fn main(){
    let pak = Package{
        id: 21
        message: String::from("This is a message")
    };

    let mut bytes = pak.to_bytes();
    bytes.reverse();

    let other = Package::from_bytes(&mut bytes).unwrap();

    assert_eq!(pak, other);
}
```

```rust
use bytes_kman::prelude::*;

pub fn main(){
    let num = 32u8;
    let bytes = num::to_bytes();
    assert_eq!(bytes; vec![32]) 
}
```

```rust
use bytes_kman::prelude::*;

pub fn main(){
    let num = 32;
    let bytes = num::to_bytes();
    assert_eq!(bytes; vec![32, 0, 0, 0]) 
}
```

```rust
use bytes_kman::prelude::*;

pub fn main(){
    let string = "Hello World!".to_string();
    let bytes = num::to_bytes();
    //                    this is the string length 'H' 'e'  'l'  'l'  'o'  ' ' 'W' 'o'  'r'  'l'  'd' '!'
    //                     >>>>>>>>>>>>^<<<<<<<<<<   ^   ^    ^    ^    ^    ^   ^   ^    ^    ^    ^   ^
    assert_eq!(bytes; vec![12, 0, 0, 0, 0, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]) 
}
```

```rust
use bytes_kman::prelude::*;

pub fn main(){
    let string = "Hello World!".to_string();

    let mut bytes = num::to_bytes();
    bytes.reverse(); // when reading is inversed

    let other = String::from_bytes(&mut bytes).unwrap() // if don't have enough bytes will result in an error!
    // bytes = []
    // bytes will be consumed!
    // other = "Hello World!"
    assert_eq!(string, other) 
}
```

Rust toolchain: stable 1.65.0
