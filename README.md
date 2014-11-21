# ansi.rs

A utility library in Rust to wrap blobs in ANSI character codes, giving them color and style.

Built using [Cargo](https://github.com/rust-lang/cargo).

## Usage

All styles can chain into new styles, by calling a relevant method. Styles can also be added together to create new ones (`Red + Strike`).

```rust
#![feature(unboxed_closures)]
extern crate ansi;

fn main() {
    println!("{} {}!", ansi::Blue("Hello"), ansi::Red.bold()("World"));
}
```

## TODO

- Docs
- Faster

## License

[MPLv2.0](./LICENSE)
