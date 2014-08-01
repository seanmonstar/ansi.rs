use std::fmt;

#[deriving(Show, Clone, PartialEq)]
pub struct Ansi {
    pub open: Code,
    pub close: Code,
}

#[deriving(PartialEq, Clone)]
pub struct Code(u8);

impl fmt::Show for Code {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let Code(c) = *self;
        write!(fmt, "\u001b[{}m", c)
    }
}

macro_rules! ansi (
    ($($name:ident: $open:expr, $close:expr,)*) => {
        $(pub static $name: Ansi = Ansi {
            open: Code($open),
            close: Code($close)
        };)*
    }
)

ansi! {
    BOLD:       1, 22, // 21 isn't widely supported and 22 does the same thing
    DIM:        2, 22,
    ITALIC:     3, 23,
    UNDERLINE:  4, 24,
    INVERSE:    7, 27,
    HIDDEN:     8, 28,
    STRIKE:     9, 29,

    BLACK:     30, 39,
    RED:       31, 39,
    GREEN:     32, 39,
    YELLOW:    33, 39,
    BLUE:      34, 39,
    MAGENTA:   35, 39,
    CYAN:      36, 39,
    WHITE:     37, 39,
    GRAY:      90, 39,

    BG_BLACK:  40, 49,
    BG_RED:    41, 49,
    BG_GREEN:  42, 49,
    BG_YELLOW: 43, 49,
    BG_BLUE:   44, 49,
    BG_MAGENTA:45, 49,
    BG_CYAN:   46, 49,
    BG_WHITE:  47, 49,
}
