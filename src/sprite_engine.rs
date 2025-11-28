use crate::{rico_engine::{PixelsType, ScreenEngine, SCREEN_SIZE}, utils::{colors::COLORS, mouse::MousePress, pixels::clear}};

pub struct SpriteEngine{
    pixels: PixelsType,
    pub mouse: MousePress,
}

impl SpriteEngine{
    pub fn new() -> Self{
        SpriteEngine { 
            pixels: vec![vec![COLORS::BLACK; SCREEN_SIZE]; SCREEN_SIZE * 2],
            mouse: MousePress::default(),
        }
    }

    pub fn update(&mut self) {
        clear(&mut self.pixels, COLORS::GRAY);
        println!("{:?}", self.mouse);

        if self.mouse.just_pressed {
            self.mouse.just_pressed = false;
        };
    }
}

impl ScreenEngine for SpriteEngine{
    type Pixels<'a> = &'a PixelsType;
    fn pixels(&self) -> Self::Pixels<'_> {
        &self.pixels
    }
}
