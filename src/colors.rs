#[derive(Debug, Clone, Copy)]
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
}

