use crate::engine::rico::{PixelsType, SCREEN_SIZE};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum COLORS {
    BLANK = 0,
    BLACK,
    WHITE,
    GRAY,
    SILVER,
    RED,
    MAROON,
    ORANGE,
    YELLOW,
    GOLD,
    GREEN,
    OLIVE,
    BROWN,
    BLUE,
    TEAL,
    PURPLE,
    PINK,
}

impl COLORS {
    pub fn pixels() -> PixelsType {
        vec![vec![COLORS::BLACK; SCREEN_SIZE]; SCREEN_SIZE]
    }

    pub fn rgba(self) -> (u8, u8, u8, u8) {
        match self {
            COLORS::BLANK  => (0, 0, 0, 0),
            COLORS::BLACK  => (0, 0, 0, 255),
            COLORS::WHITE  => (255, 255, 255, 255),
            COLORS::GRAY   => (128, 128, 128, 255),
            COLORS::SILVER => (192, 192, 192, 255),
            COLORS::RED    => (200, 40, 40, 255),
            COLORS::MAROON => (128, 0, 0, 255),
            COLORS::ORANGE => (255, 140, 0, 255),
            COLORS::YELLOW => (240, 230, 80, 255),
            COLORS::GOLD   => (255, 215, 0, 255),
            COLORS::GREEN  => (0, 180, 0, 255),
            COLORS::OLIVE  => (128, 128, 0, 255),
            COLORS::BROWN  => (139, 69, 19, 255),
            COLORS::BLUE   => (65, 105, 225, 255),
            COLORS::TEAL   => (0, 128, 128, 255),
            COLORS::PURPLE => (138, 43, 226, 255),
            COLORS::PINK   => (255, 105, 180, 255),
        }
    }
}

impl fmt::Display for COLORS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
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
        };
        write!(f, "{s}")
    }
}

impl FromStr for COLORS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BLACK"  => Ok(COLORS::BLACK),
            "WHITE"  => Ok(COLORS::WHITE),
            "GRAY"   => Ok(COLORS::GRAY),
            "SILVER" => Ok(COLORS::SILVER),
            "RED"    => Ok(COLORS::RED),
            "MAROON" => Ok(COLORS::MAROON),
            "ORANGE" => Ok(COLORS::ORANGE),
            "YELLOW" => Ok(COLORS::YELLOW),
            "GOLD"   => Ok(COLORS::GOLD),
            "GREEN"  => Ok(COLORS::GREEN),
            "OLIVE"  => Ok(COLORS::OLIVE),
            "BROWN"  => Ok(COLORS::BROWN),
            "BLUE"   => Ok(COLORS::BLUE),
            "TEAL"   => Ok(COLORS::TEAL),
            "PURPLE" => Ok(COLORS::PURPLE),
            "PINK"   => Ok(COLORS::PINK),
            "BLANK"  => Ok(COLORS::BLANK),
            _        => Err(()),
        }
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
