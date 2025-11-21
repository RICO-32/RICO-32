use crate::goon_engine::{PixelsType, SCREEN_SIZE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct COLORS(pub u8, pub u8, pub u8);

impl COLORS {
    pub const BLACK: COLORS   = COLORS(0, 0, 0);
    pub const WHITE: COLORS   = COLORS(255, 255, 255);
    pub const RED: COLORS     = COLORS(255, 0, 0);
    pub const LIME: COLORS    = COLORS(0, 255, 0);
    pub const BLUE: COLORS    = COLORS(0, 0, 255);
    pub const YELLOW: COLORS  = COLORS(255, 255, 0);
    pub const CYAN: COLORS    = COLORS(0, 255, 255);
    pub const MAGENTA: COLORS = COLORS(255, 0, 255);
    pub const SILVER: COLORS  = COLORS(192, 192, 192);
    pub const GRAY: COLORS    = COLORS(128, 128, 128);
    pub const MAROON: COLORS  = COLORS(128, 0, 0);
    pub const OLIVE: COLORS   = COLORS(128, 128, 0);
    pub const GREEN: COLORS   = COLORS(0, 128, 0);
    pub const PURPLE: COLORS  = COLORS(128, 0, 128);
    pub const TEAL: COLORS    = COLORS(0, 128, 128);
    pub const NAVY: COLORS    = COLORS(0, 0, 128);

    pub fn pixels() -> PixelsType{
        [[COLORS::BLACK; SCREEN_SIZE as usize]; SCREEN_SIZE as usize]
    }
}

pub fn color_from_str(name: &str) -> Option<COLORS> {
    match name.to_uppercase().as_str() {
        "BLACK"   => Some(COLORS::BLACK),
        "WHITE"   => Some(COLORS::WHITE),
        "RED"     => Some(COLORS::RED),
        "LIME"    => Some(COLORS::LIME),
        "BLUE"    => Some(COLORS::BLUE),
        "YELLOW"  => Some(COLORS::YELLOW),
        "CYAN"    => Some(COLORS::CYAN),
        "MAGENTA" => Some(COLORS::MAGENTA),
        "SILVER"  => Some(COLORS::SILVER),
        "GRAY"    => Some(COLORS::GRAY),
        "MAROON"  => Some(COLORS::MAROON),
        "OLIVE"   => Some(COLORS::OLIVE),
        "GREEN"   => Some(COLORS::GREEN),
        "PURPLE"  => Some(COLORS::PURPLE),
        "TEAL"    => Some(COLORS::TEAL),
        "NAVY"    => Some(COLORS::NAVY),
        _         => None,
    }
}

pub fn str_from_color(col: COLORS) -> String {
    match col {
        COLORS::BLACK => String::from("BLACK"),
        COLORS::WHITE => String::from("WHITE"),
        COLORS::RED => String::from("RED"),
        COLORS::LIME => String::from("LIME"),
        COLORS::BLUE => String::from("BLUE"),
        COLORS::YELLOW => String::from("YELLOW"),
        COLORS::CYAN => String::from("CYAN"),
        COLORS::MAGENTA => String::from("MAGENTA"),
        COLORS::SILVER => String::from("SILVER"),
        COLORS::GRAY => String::from("GRAY"),
        COLORS::MAROON => String::from("MAROON"),
        COLORS::OLIVE => String::from("OLIVE"),
        COLORS::GREEN => String::from("GREEN"),
        COLORS::PURPLE => String::from("PURPLE"),
        COLORS::TEAL => String::from("TEAL"),
        COLORS::NAVY => String::from("NAVY"),
        _ => String::from("")
    }
}
