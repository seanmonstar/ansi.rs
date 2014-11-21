#![feature(macro_rules, unboxed_closures)]
#![deny(warnings)]
#![experimental]

//! # Ansi
//! Easily build up ansi styles that can be written to a Formatter.
//!
//! ### Examples
//!
//! ```rust
//! #![feature(unboxed_closures)]
//! extern crate ansi;
//!
//! use ansi::{Bold, Red, White};
//!
//! fn main() {
//!
//!     // any style can be added to others
//!     let error = Red + Bold;
//!     println!("{}", error("red and bold"));
//!
//!     // or you can change methods
//!     let critical = White.bg_red().bold();
//!     println!("{}", critical("bold, white, and red bg"));
//! }
//! ```

#[cfg(test)] extern crate test;

pub use fmt::{
    Style,
    Styled,

    Bold,
    Dim,
    Italic,
    Underline,
    Inverse,
    Hidden,
    Strike,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite
};

pub mod ansi;
mod fmt;


#[cfg(test)]
mod tests {

    use std::fmt::Show;
    use test::Bencher;

    #[inline]
    fn assert_show<T: Show>(show: T, expected: &str) {
        assert_eq!(format!("{}", show).as_slice(), expected);
    }

    #[test]
    fn test_colors_string() {
        assert_show(super::Red("red"), "\u001b[31mred\u001b[39m");
    }

    #[test]
    fn test_colors_add() {
        let color = super::Red + super::Bold;
        assert_show(color("red"), "\u001b[1m\u001b[31mred\u001b[39m\u001b[22m");
        let color2 = color + super::BgYellow;
        assert_show(color2("red"), "\u001b[43m\u001b[1m\u001b[31mred\u001b[39m\u001b[22m\u001b[49m");
    }

    #[test]
    fn test_colors_chain() {
        let color = super::Red.bold();
        assert_show(color("red"), "\u001b[1m\u001b[31mred\u001b[39m\u001b[22m");
        let color2 = color.bg_yellow();
        assert_show(color2("red"), "\u001b[43m\u001b[1m\u001b[31mred\u001b[39m\u001b[22m\u001b[49m");
    }

    #[bench]
    fn bench_fmt(b: &mut Bencher) {
        let red = super::Red("red");
        b.iter(|| {
            format!("{}", red);
        });
    }

    #[bench]
    fn bench_fmt_many_styles(b: &mut Bencher) {
        let many = super::Red.bold().underline().bg_blue()("many");
        b.iter(|| {
            format!("{}", many);
        });
    }

    #[bench]
    fn bench_builder(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::Red.bold().bg_blue()("red");
        });
    }
}
