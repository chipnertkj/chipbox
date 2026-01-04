//! Select Graphic Rendition (SGR) codes.
//!
//! See: <https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters>

use ratatui::style::Color;

pub const fn fg_color(param: u16) -> Option<Color> {
    let fg = match param {
        FG_BLACK => Color::Black,
        FG_RED => Color::Red,
        FG_GREEN => Color::Green,
        FG_YELLOW => Color::Yellow,
        FG_BLUE => Color::Blue,
        FG_MAGENTA => Color::Magenta,
        FG_CYAN => Color::Cyan,
        FG_WHITE => Color::White,
        _ => return None,
    };
    Some(fg)
}

pub const fn bg_color(param: u16) -> Option<Color> {
    let bg = match param {
        BG_BLACK => Color::Black,
        BG_RED => Color::Red,
        BG_GREEN => Color::Green,
        BG_YELLOW => Color::Yellow,
        BG_BLUE => Color::Blue,
        BG_MAGENTA => Color::Magenta,
        BG_CYAN => Color::Cyan,
        BG_WHITE => Color::White,
        _ => return None,
    };
    Some(bg)
}

pub const ACTION: char = 'm';

pub const RESET: u16 = 0;
pub const BOLD: u16 = 1;
pub const ITALIC: u16 = 3;
pub const UNDERLINE: u16 = 4;

pub const FG_FIRST: u16 = FG_BLACK;
pub const FG_LAST: u16 = FG_WHITE;
pub const FG_BLACK: u16 = 30;
pub const FG_RED: u16 = 31;
pub const FG_GREEN: u16 = 32;
pub const FG_YELLOW: u16 = 33;
pub const FG_BLUE: u16 = 34;
pub const FG_MAGENTA: u16 = 35;
pub const FG_CYAN: u16 = 36;
pub const FG_WHITE: u16 = 37;
pub const FG_RGB: u16 = 38;

pub const BG_FIRST: u16 = BG_BLACK;
pub const BG_LAST: u16 = BG_WHITE;
pub const BG_BLACK: u16 = 40;
pub const BG_RED: u16 = 41;
pub const BG_GREEN: u16 = 42;
pub const BG_YELLOW: u16 = 43;
pub const BG_BLUE: u16 = 44;
pub const BG_MAGENTA: u16 = 45;
pub const BG_CYAN: u16 = 46;
pub const BG_WHITE: u16 = 47;
pub const BG_RGB: u16 = 48;

pub const RGB_DIRECT: u16 = 2;
pub const RGB_256_PALETTE: u16 = 5;
