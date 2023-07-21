![Crates.io](https://img.shields.io/crates/l/lilbig) ![Crates.io](https://img.shields.io/crates/v/lilbig) ![docs.rs](https://img.shields.io/docsrs/lilbig)

# LilBig
A Rust crate offering utilities for swapping the byte-order of in-memory types.

## 🚧 Stability 🚧
This crate is in early development. Understand that breaking changes may be made between minor versions tagged with the pre-release code `alpha`. Additionally understand that if you are interested in using this crate but have some dissatisfaction with the current API that now is an excellent time to make [suggestions](https://github.com/IanE9/lilbig/issues).

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
