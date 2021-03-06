use std::fmt;
use std::ops::{Add/*, Fn*/};
use std::slice::Items;

use ansi::{mod, Ansi};

pub struct Style<V> {
    styles: V
}



impl<VL: AsSlice<Ansi>, VR: AsSlice<Ansi>> Add<Style<VR>, Style<Vec<Ansi>>> for Style<VL> {
    fn add(&self, other: &Style<VR>) -> Style<Vec<Ansi>> {
        let (left, right) = (self.styles.as_slice(), other.styles.as_slice());

        let mut styles = Vec::with_capacity(left.len() + right.len());
        let mut iter = left.iter().chain(right.iter());
        for &ansi in iter {
            styles.push(ansi);
        }
        Style {
            styles: styles
        }
    }
}


impl<T: fmt::Show, V: AsSlice<Ansi>> Style<V> {
    pub fn show(&self, show: T) -> Styled<T> {
        Styled {
            styles: self.styles.as_slice().to_vec(),
            subject: show
        }
    }
}

impl<T: fmt::Show, V: AsSlice<Ansi>> Fn<(T,), Styled<T>> for Style<V> {
    extern "rust-call" fn call(&self, (show,): (T,)) -> Styled<T> {
        self.show(show)
    }
}

pub struct Styled<T> {
    styles: Vec<Ansi>,
    subject: T
}

impl<T: fmt::Show> fmt::Show for Styled<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        return recurse(self.styles.as_slice().iter(), &self.subject, fmt);

        fn recurse<'a, T: fmt::Show>(mut styles: Items<'a, Ansi>, subject: &T, fmt: &mut fmt::Formatter) -> fmt::Result {
            match styles.next_back() {
                None => subject.fmt(fmt),
                Some(style) => {
                    try!(style.open.fmt(fmt));
                    try!(recurse(styles, subject, fmt));
                    style.close.fmt(fmt)
                }
            }
        }
    }
}


macro_rules! styles (
    ($($name:ident: $ansi:ident, $method:ident),*) => {
        $(
        #[allow(non_upper_case_globals)]
        pub static $name: Style<[Ansi, ..1]> = Style {
            styles: [ansi::$ansi]
        };
        )*

        impl<V: AsSlice<Ansi>> Style<V> {
            $(
            pub fn $method(&self) -> Style<Vec<Ansi>> {
                *self + $name
            }
            )*
        }
    }
)

styles! {
    Bold: BOLD, bold,
    Dim: DIM, dim,
    Italic: ITALIC, italic,
    Underline: UNDERLINE, underline,
    Inverse: INVERSE, inverse,
    Hidden: HIDDEN, hidden,
    Strike: STRIKE, strike,

    Black: BLACK, black,
    Red: RED, red,
    Green: GREEN, green,
    Yellow: YELLOW, yellow,
    Blue: BLUE, blue,
    Magenta: MAGENTA, magenta,
    Cyan: CYAN, cyan,
    White: WHITE, white,
    Gray: GRAY, gray,

    BgBlack: BG_BLACK, bg_black,
    BgRed: BG_RED, bg_red,
    BgGreen: BG_GREEN, bg_green,
    BgYellow: BG_YELLOW, bg_yellow,
    BgBlue: BG_BLUE, bg_blue,
    BgMagenta: BG_MAGENTA, bg_magenta,
    BgCyan: BG_CYAN, bg_cyan,
    BgWhite: BG_WHITE, bg_white
}
