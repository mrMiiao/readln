[![crates.io](https://img.shields.io/crates/v/readln.svg)](https://crates.io/crates/readln)
[![License](https://img.shields.io/crates/l/readln.svg)](https://choosealicense.com/licenses/mpl-2.0/)
[![Documentation](https://img.shields.io/docsrs/readln/latest)](https://docs.rs/readln)

# readln

__*readln*__ crate provides you access to `read` macro.

Example:

```rust
#[macro_use]
extern crate readln;

fn main() {
    let _x: &str = read!().unwrap();
    let _y: u8 = read!(u8).unwrap();
}
```