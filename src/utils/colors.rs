use crate::rico_engine::{PixelsType, SCREEN_SIZE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct COLORS(pub u8, pub u8, pub u8, pub u8);

impl COLORS {
    pub const BLANK: COLORS   = COLORS(0, 0, 0, 0);

    pub const BLACK: COLORS  = COLORS(0, 0, 0, 255);
    pub const WHITE: COLORS  = COLORS(255, 255, 255, 255);
    pub const GRAY: COLORS   = COLORS(128, 128, 128, 255);
    pub const SILVER: COLORS = COLORS(192, 192, 192, 255);
    pub const RED: COLORS    = COLORS(200, 40, 40, 255);
    pub const MAROON: COLORS = COLORS(128, 0, 0, 255);
    pub const ORANGE: COLORS = COLORS(255, 140, 0, 255);
    pub const YELLOW: COLORS = COLORS(240, 230, 80, 255);
    pub const GOLD: COLORS   = COLORS(255, 215, 0, 255);
    pub const GREEN: COLORS  = COLORS(0, 180, 0, 255);
    pub const OLIVE: COLORS  = COLORS(128, 128, 0, 255);
    pub const BROWN: COLORS  = COLORS(139, 69, 19, 255);
    pub const BLUE: COLORS   = COLORS(65, 105, 225, 255);
    pub const TEAL: COLORS   = COLORS(0, 128, 128, 255);
    pub const PURPLE: COLORS = COLORS(138, 43, 226, 255);
    pub const PINK: COLORS   = COLORS(255, 105, 180, 255);

    pub fn pixels() -> PixelsType {
        vec![vec![COLORS::BLACK; SCREEN_SIZE]; SCREEN_SIZE]
    }
}

pub const ALL_COLORS: [COLORS; 17] = [
    COLORS::BLANK,

    COLORS::BLACK,
    COLORS::WHITE,
    COLORS::GRAY,
    COLORS::SILVER,
    COLORS::RED,
    COLORS::MAROON,
    COLORS::ORANGE,
    COLORS::YELLOW,
    COLORS::GOLD,
    COLORS::GREEN,
    COLORS::OLIVE,
    COLORS::BROWN,
    COLORS::BLUE,
    COLORS::TEAL,
    COLORS::PURPLE,
    COLORS::PINK,
];

pub fn color_from_str(name: &str) -> Option<COLORS> {
    match name.to_uppercase().as_str() {
        "BLACK"  => Some(COLORS::BLACK),
        "WHITE"  => Some(COLORS::WHITE),
        "GRAY"   => Some(COLORS::GRAY),
        "SILVER" => Some(COLORS::SILVER),
        "RED"    => Some(COLORS::RED),
        "MAROON" => Some(COLORS::MAROON),
        "ORANGE" => Some(COLORS::ORANGE),
        "YELLOW" => Some(COLORS::YELLOW),
        "GOLD"   => Some(COLORS::GOLD),
        "GREEN"  => Some(COLORS::GREEN),
        "OLIVE"  => Some(COLORS::OLIVE),
        "BROWN"  => Some(COLORS::BROWN),
        "BLUE"   => Some(COLORS::BLUE),
        "TEAL"   => Some(COLORS::TEAL),
        "PURPLE" => Some(COLORS::PURPLE),
        "PINK"   => Some(COLORS::PINK),
        "BLANK"  => Some(COLORS::BLANK),
        _        => None,
    }
}

pub fn str_from_color(col: COLORS) -> String {
    match col {
        COLORS::BLACK  => "BLACK",
        COLORS::WHITE  => "WHITE",
        COLORS::GRAY   => "GRAY",
        COLORS::SILVER => "SILVER",
        COLORS::RED    => "RED",
        COLORS::MAROON => "MAROON",
        COLORS::ORANGE => "ORANGE",
        COLORS::YELLOW => "YELLOW",
        COLORS::GOLD   => "GOLD",
        COLORS::GREEN  => "GREEN",
        COLORS::OLIVE  => "OLIVE",
        COLORS::BROWN  => "BROWN",
        COLORS::BLUE   => "BLUE",
        COLORS::TEAL   => "TEAL",
        COLORS::PURPLE => "PURPLE",
        COLORS::PINK   => "PINK",
        COLORS::BLANK  => "BLANK",
        _              => "",
    }
    .to_string()
}

