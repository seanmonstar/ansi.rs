# colors.rs

A utility library in Rust to wrap blobs in ANSI character codes, giving them color and style.

Built using [Cargo][https://github.com/rust-lang/cargo].

## Usage

All styles can chain into new styles, by calling a relevant method. Styles can also be added together to create new ones (`Red + Strike`).

```rust
extern crate colors;

fn main() {
    println!("{} {}!", colors::Blue.show("Hello"), colors::Red.bold().show("World"));
}
```

## TODO

- impl `Fn` on `Style`, so you can call it directly instead of using a `show` method.
    - The code to do this is already there, but blocked on a `rustc` bug: https://github.com/rust-lang/rust/issues/15905
- Docs
- Faster

## License

[MPLv2.0](./LICENSE)
