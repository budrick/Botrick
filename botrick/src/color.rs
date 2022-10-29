use std::fmt;
use std::fmt::{Formatter, Result};

#[allow(dead_code)]
pub enum Color {
    White,
    Black,
    Blue,
    Green,
    Red,
    Brown,
    Magenta,
    Orange,
    Yellow,
    LightGreen,
    Cyan,
    LightCyan,
    LightBlue,
    Pink,
    Gray,
    LightGray,
    Default,
    Num(u8),
    Reset,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Color::White => write!(f, "00"),
            Color::Black => write!(f, "01"),
            Color::Blue => write!(f, "02"),
            Color::Green => write!(f, "03"),
            Color::Red => write!(f, "04"),
            Color::Brown => write!(f, "05"),
            Color::Magenta => write!(f, "06"),
            Color::Orange => write!(f, "07"),
            Color::Yellow => write!(f, "08"),
            Color::LightGreen => write!(f, "09"),
            Color::Cyan => write!(f, "10"),
            Color::LightCyan => write!(f, "11"),
            Color::LightBlue => write!(f, "12"),
            Color::Pink => write!(f, "13"),
            Color::Gray => write!(f, "14"),
            Color::LightGray => write!(f, "15"),
            Color::Default => write!(f, "99"),
            Color::Num(num) => write!(f, "{:02}", num),
            Color::Reset => write!(f, ""),
        }
    }
}

pub fn colors(fg: Color, bg: Option<Color>) -> String {
    if matches!(fg, Color::Reset) {
        return String::from("\u{3}");
    }

    if bg.is_some() {
        format!("\u{3}{},{}", fg, bg.unwrap())
    } else {
        format!("\u{3}{}", fg)
    }
}

pub fn colorize(fg: Color, bg: Option<Color>, message: &str) -> String {
    format!(
        "{}{}{}",
        colors(fg, bg),
        message,
        colors(Color::Reset, None)
    )
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn test_fg_only() {
        let result = crate::color::colorize(Color::Red, None, "hi");
        assert_eq!(result.as_str(), "\u{3}04hi\u{3}");
    }

    #[test]
    fn test_fg_and_bg() {
        let result = crate::color::colorize(Color::Red, Some(Color::Green), "hi");
        assert_eq!(result.as_str(), "\u{3}04,03hi\u{3}");
    }
}
