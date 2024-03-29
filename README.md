# LilBig
[![Crates.io](https://img.shields.io/crates/v/lilbig)](https://crates.io/crates/lilbig) [![Crates.io](https://img.shields.io/crates/l/lilbig)](https://choosealicense.com/licenses/mpl-2.0/) [![docs.rs](https://img.shields.io/docsrs/lilbig)](https://docs.rs/lilbig/)

A Rust crate offering utilities for swapping the byte-order of in-memory types.

## When might I want to use this crate?
The user will likely find this crate most useful and appropriate when the following conditions are met:
* The data being operated on is trivially loadable into memory.
* The data being operated on does not have a byte-order that can be known at compile-time.

## When might I *not* want to use this crate?
If any of the previously mentioned conditions are not met, then the user will likely find using this crate to feel clunky.

## The crate doesn't seem appropriate. What might I use instead?
If the user feels this crate is inappropriate, then they might consider the following alternatives:
* [The Standard Library](https://doc.rust-lang.org/std/primitive.u32.html#method.swap_bytes)
* [LEBE](https://github.com/johannesvollmer/lebe)
* [byteorder](https://github.com/BurntSushi/byteorder)
