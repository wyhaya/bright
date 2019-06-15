#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    FastBlink,
    Invert,
    CrossOut,
}

#[derive(Debug)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
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
            Style::CrossOut => 9,
        };
        Some(n)
    } else {
        None
    }
}

fn get_color(color: &Option<Color>) -> Option<usize> {
    if let Some(color) = color {
        let n = match color {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
        };
        Some(n)
    } else {
        None
    }
}

fn get_background_color(color: &Option<Color>) -> Option<usize> {
    if let Some(n) = get_color(color) {
        Some(n + 10)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Hue {
    text: String,
    color: Option<Color>,
    background: Option<Color>,
    style: Option<Style>,
}

impl Default for Hue {
    fn default() -> Self {
        Hue {
            text: String::default(),
            color: None,
            background: None,
            style: None,
        }
    }
}

impl fmt::Display for Hue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ops = vec![
            get_style(&self.style),
            get_color(&self.color),
            get_background_color(&self.background),
        ]
        .iter()
        .map(|item| match item {
            Some(n) => n.to_string(),
            None => String::new(),
        })
        .filter(|item| item != "")
        .collect::<Vec<String>>()
        .join(";");

        let data = format!("\x1B[{}m{}\x1B[0m", ops, self.text);

        write!(f, "{}", data)
    }
}

macro_rules! def_str_style {
    ($name: ident, $style: path) => {
        fn $name(self) -> Hue {
            Hue {
                text: String::from(self),
                style: Some($style),
                ..Hue::default()
            }
        }
    };
}

macro_rules! def_str_color {
    ($name: ident, $color: path) => {
        fn $name(self) -> Hue {
            Hue {
                text: String::from(self),
                color: Some($color),
                ..Hue::default()
            }
        }
    };
}

macro_rules! def_str_background_color {
    ($name: ident, $color: path) => {
        fn $name(self) -> Hue {
            Hue {
                text: String::from(self),
                background: Some($color),
                ..Hue::default()
            }
        }
    };
}

macro_rules! def_style {
    ($name: ident, $style: path) => {
        pub fn $name(self) -> Hue {
             Hue {
                style: Some($style),
                ..self
            }
        }
    };
}

macro_rules! def_color {
    ($name: ident, $color: path) => {
        pub fn $name(self) -> Hue {
            Hue {
                color: Some($color),
                ..self
            }
        }
    };
}

macro_rules! def_background_color {
    ($name: ident, $color: path) => {
        pub fn $name(self) -> Hue {
             Hue {
                background: Some($color),
                ..self
            }
        }
    };
}

pub trait Hues {
    fn bold(self) -> Hue;
    fn dim(self) -> Hue;
    fn italic(self) -> Hue;
    fn underline(self) -> Hue;
    fn slow_blink(self) -> Hue;
    fn fast_blink(self) -> Hue;
    fn invert(self) -> Hue;
    fn cross_out(self) -> Hue;

    fn black(self) -> Hue;
    fn red(self) -> Hue;
    fn green(self) -> Hue;
    fn yellow(self) -> Hue;
    fn blue(self) -> Hue;
    fn magenta(self) -> Hue;
    fn cyan(self) -> Hue;
    fn white(self) -> Hue;

    fn background_black(self) -> Hue;
    fn background_red(self) -> Hue;
    fn background_green(self) -> Hue;
    fn background_yellow(self) -> Hue;
    fn background_blue(self) -> Hue;
    fn background_magenta(self) -> Hue;
    fn background_cyan(self) -> Hue;
    fn background_white(self) -> Hue;
}

impl Hue {
    def_style!(bold, Style::Bold);
    def_style!(dim, Style::Dim);
    def_style!(italic, Style::Italic);
    def_style!(underline, Style::Underline);
    def_style!(slow_blink, Style::SlowBlink);
    def_style!(fast_blink, Style::FastBlink);
    def_style!(invert, Style::Invert);
    def_style!(cross_out, Style::CrossOut);

    def_color!(black, Color::Black);
    def_color!(red, Color::Red);
    def_color!(green, Color::Green);
    def_color!(yellow, Color::Yellow);
    def_color!(blue, Color::Blue);
    def_color!(magenta, Color::Magenta);
    def_color!(cyan, Color::Cyan);
    def_color!(white, Color::White);

    def_background_color!(background_black, Color::Black);
    def_background_color!(background_red, Color::Red);
    def_background_color!(background_green, Color::Green);
    def_background_color!(background_yellow, Color::Yellow);
    def_background_color!(background_blue, Color::Blue);
    def_background_color!(background_magenta, Color::Magenta);
    def_background_color!(background_cyan, Color::Cyan);
    def_background_color!(background_white, Color::White);
}

impl<'a> Hues for &'a str {
    def_str_style!(bold, Style::Bold);
    def_str_style!(dim, Style::Dim);
    def_str_style!(italic, Style::Italic);
    def_str_style!(underline, Style::Underline);
    def_str_style!(slow_blink, Style::SlowBlink);
    def_str_style!(fast_blink, Style::FastBlink);
    def_str_style!(invert, Style::Invert);
    def_str_style!(cross_out, Style::CrossOut);

    def_str_color!(black, Color::Black);
    def_str_color!(red, Color::Red);
    def_str_color!(green, Color::Green);
    def_str_color!(yellow, Color::Yellow);
    def_str_color!(blue, Color::Blue);
    def_str_color!(magenta, Color::Magenta);
    def_str_color!(cyan, Color::Cyan);
    def_str_color!(white, Color::White);

    def_str_background_color!(background_black, Color::Black);
    def_str_background_color!(background_red, Color::Red);
    def_str_background_color!(background_green, Color::Green);
    def_str_background_color!(background_yellow, Color::Yellow);
    def_str_background_color!(background_blue, Color::Blue);
    def_str_background_color!(background_magenta, Color::Magenta);
    def_str_background_color!(background_cyan, Color::Cyan);
    def_str_background_color!(background_white, Color::White);
}
