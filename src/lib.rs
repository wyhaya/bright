#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    FastBlink,
    Invert,
    Hidden,
    CrossOut,
}

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Rgb(usize, usize, usize),
}

fn get_style(style: &Option<Style>) -> Option<usize> {
    if let Some(style) = style {
        let n = match style {
            Style::Bold => 1,
            Style::Dim => 2,
            Style::Italic => 3,
            Style::Underline => 4,
            Style::SlowBlink => 5,
            Style::FastBlink => 6,
            Style::Invert => 7,
            Style::Hidden => 8,
            Style::CrossOut => 9,
        };
        return Some(n);
    }
    None
}

fn get_color(color: &Option<Color>) -> Option<Vec<usize>> {
    if let Some(color) = color {
        let n = match color {
            Color::Black => vec![30],
            Color::Red => vec![31],
            Color::Green => vec![32],
            Color::Yellow => vec![33],
            Color::Blue => vec![34],
            Color::Magenta => vec![35],
            Color::Cyan => vec![36],
            Color::White => vec![37],
            Color::Rgb(r, g, b) => vec![38, 2, *r, *g, *b],
        };
        return Some(n);
    }
    None
}

fn get_background_color(color: &Option<Color>) -> Option<Vec<usize>> {
    if let Some(color) = color {
        let n = match color {
            Color::Black => vec![40],
            Color::Red => vec![41],
            Color::Green => vec![42],
            Color::Yellow => vec![43],
            Color::Blue => vec![44],
            Color::Magenta => vec![45],
            Color::Cyan => vec![46],
            Color::White => vec![47],
            Color::Rgb(r, g, b) => vec![48, 2, *r, *g, *b],
        };
        Some(n)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Bright {
    text: String,
    color: Option<Color>,
    background: Option<Color>,
    style: Option<Style>,
}

impl Bright {
    pub fn new<T: ToString>(text: T) -> Bright {
        Bright {
            text: text.to_string(),
            color: None,
            background: None,
            style: None,
        }
    }
}

impl Default for Bright {
    fn default() -> Self {
        Bright {
            text: String::default(),
            color: None,
            background: None,
            style: None,
        }
    }
}

fn merge_color(color: Option<Vec<usize>>) -> Option<String> {
    match color {
        Some(vec) => {
            let s = vec
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(";");
            Some(s)
        }
        None => None,
    }
}

impl fmt::Display for Bright {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = match get_style(&self.style) {
            Some(d) => Some(d.to_string()),
            None => None,
        };

        let all = vec![
            style,
            merge_color(get_color(&self.color)),
            merge_color(get_background_color(&self.background)),
        ];

        let ops = all
            .iter()
            .filter_map(|item| match item {
                Some(d) => Some(d.clone()),
                None => None,
            })
            .collect::<Vec<String>>()
            .join(";");

        let output = format!("\x1B[{}m{}\x1B[0m", ops, self.text);

        write!(f, "{}", output)
    }
}

macro_rules! def_str_style {
    ($name: ident, $style: path) => {
        fn $name(self) -> Bright {
            Bright {
                text: String::from(self),
                style: Some($style),
                ..Bright::default()
            }
        }
    };
}

macro_rules! def_str_color {
    ($name: ident, $color: path) => {
        fn $name(self) -> Bright {
            Bright {
                text: String::from(self),
                color: Some($color),
                ..Bright::default()
            }
        }
    };
}

macro_rules! def_str_background_color {
    ($name: ident, $color: path) => {
        fn $name(self) -> Bright {
            Bright {
                text: String::from(self),
                background: Some($color),
                ..Bright::default()
            }
        }
    };
}

macro_rules! def_style {
    ($name: ident, $style: path) => {
        pub fn $name(self) -> Bright {
             Bright {
                style: Some($style),
                ..self
            }
        }
    };
}

macro_rules! def_color {
    ($name: ident, $color: path) => {
        pub fn $name(self) -> Bright {
            Bright {
                color: Some($color),
                ..self
            }
        }
    };
}

macro_rules! def_background_color {
    ($name: ident, $color: path) => {
        pub fn $name(self) -> Bright {
             Bright {
                background: Some($color),
                ..self
            }
        }
    };
}

pub trait Colorful {
    fn bold(self) -> Bright;
    fn dim(self) -> Bright;
    fn italic(self) -> Bright;
    fn underline(self) -> Bright;
    fn slow_blink(self) -> Bright;
    fn fast_blink(self) -> Bright;
    fn invert(self) -> Bright;
    fn hidden(self) -> Bright;
    fn cross_out(self) -> Bright;

    fn black(self) -> Bright;
    fn red(self) -> Bright;
    fn green(self) -> Bright;
    fn yellow(self) -> Bright;
    fn blue(self) -> Bright;
    fn magenta(self) -> Bright;
    fn cyan(self) -> Bright;
    fn white(self) -> Bright;
    fn rgb(self, r: usize, g: usize, b: usize) -> Bright;

    fn bg_black(self) -> Bright;
    fn bg_red(self) -> Bright;
    fn bg_green(self) -> Bright;
    fn bg_yellow(self) -> Bright;
    fn bg_blue(self) -> Bright;
    fn bg_magenta(self) -> Bright;
    fn bg_cyan(self) -> Bright;
    fn bg_white(self) -> Bright;
    fn bg_rgb(self, r: usize, g: usize, b: usize) -> Bright;
}

impl Bright {
    def_style!(bold, Style::Bold);
    def_style!(dim, Style::Dim);
    def_style!(italic, Style::Italic);
    def_style!(underline, Style::Underline);
    def_style!(slow_blink, Style::SlowBlink);
    def_style!(fast_blink, Style::FastBlink);
    def_style!(invert, Style::Invert);
    def_style!(hidden, Style::Hidden);
    def_style!(cross_out, Style::CrossOut);

    def_color!(black, Color::Black);
    def_color!(red, Color::Red);
    def_color!(green, Color::Green);
    def_color!(yellow, Color::Yellow);
    def_color!(blue, Color::Blue);
    def_color!(magenta, Color::Magenta);
    def_color!(cyan, Color::Cyan);
    def_color!(white, Color::White);
    pub fn rgb(self, r: usize, g: usize, b: usize) -> Bright {
        Bright {
            color: Some(Color::Rgb(r, g, b)),
            ..self
        }
    }

    def_background_color!(bg_black, Color::Black);
    def_background_color!(bg_red, Color::Red);
    def_background_color!(bg_green, Color::Green);
    def_background_color!(bg_yellow, Color::Yellow);
    def_background_color!(bg_blue, Color::Blue);
    def_background_color!(bg_magenta, Color::Magenta);
    def_background_color!(bg_cyan, Color::Cyan);
    def_background_color!(bg_white, Color::White);
    pub fn bg_rgb(self, r: usize, g: usize, b: usize) -> Bright {
        Bright {
            background: Some(Color::Rgb(r, g, b)),
            ..self
        }
    }
}

macro_rules! def_extend {
    () => {
        def_str_style!(bold, Style::Bold);
        def_str_style!(dim, Style::Dim);
        def_str_style!(italic, Style::Italic);
        def_str_style!(underline, Style::Underline);
        def_str_style!(slow_blink, Style::SlowBlink);
        def_str_style!(fast_blink, Style::FastBlink);
        def_str_style!(invert, Style::Invert);
        def_str_style!(hidden, Style::Hidden);
        def_str_style!(cross_out, Style::CrossOut);

        def_str_color!(black, Color::Black);
        def_str_color!(red, Color::Red);
        def_str_color!(green, Color::Green);
        def_str_color!(yellow, Color::Yellow);
        def_str_color!(blue, Color::Blue);
        def_str_color!(magenta, Color::Magenta);
        def_str_color!(cyan, Color::Cyan);
        def_str_color!(white, Color::White);
        fn rgb(self, r: usize, g: usize, b: usize) -> Bright {
            Bright {
                text: String::from(self),
                color: Some(Color::Rgb(r, g, b)),
                ..Bright::default()
            }
        }

        def_str_background_color!(bg_black, Color::Black);
        def_str_background_color!(bg_red, Color::Red);
        def_str_background_color!(bg_green, Color::Green);
        def_str_background_color!(bg_yellow, Color::Yellow);
        def_str_background_color!(bg_blue, Color::Blue);
        def_str_background_color!(bg_magenta, Color::Magenta);
        def_str_background_color!(bg_cyan, Color::Cyan);
        def_str_background_color!(bg_white, Color::White);
        fn bg_rgb(self, r: usize, g: usize, b: usize) -> Bright {
            Bright {
                text: String::from(self),
                background: Some(Color::Rgb(r, g, b)),
                ..Bright::default()
            }
        }
    };
}

impl<'a> Colorful for &'a str {
    def_extend!();
}

impl Colorful for String {
    def_extend!();
}
