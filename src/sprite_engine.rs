use crate::{rico_engine::{PixelsType, ScreenEngine, SCREEN_SIZE}, utils::{colors::{ALL_COLORS, COLORS}, mouse::MousePress, pixels::{clear, rect, rect_fill, set_pix}}};

pub struct SpriteEngine{
    pixels: PixelsType,
    selected_color: COLORS,
    pub mouse: MousePress,
    pub sprite_pixs: PixelsType
}

const COLOR_BUTTON_WIDTH: i32 = 12;
const SPRITE_SIZE: usize = 32;

impl SpriteEngine{
    pub fn new() -> Self{
        SpriteEngine { 
            pixels: vec![vec![COLORS::BLACK; SCREEN_SIZE]; SCREEN_SIZE * 2],
            mouse: MousePress::default(),
            selected_color: COLORS::BLACK,
            sprite_pixs: vec![vec![COLORS::BLANK; SPRITE_SIZE]; SPRITE_SIZE],
        }
    }

    fn draw_canvas(&mut self) {
        for y in 0..SPRITE_SIZE as i32{
            for x in 0..SPRITE_SIZE as i32{
                let col = self.sprite_pixs[y as usize][x as usize];
                if col == COLORS::BLANK {
                    if (y + x) % 2 == 0 {
                        rect_fill(&mut self.pixels, 32+x*2, 64+y*2, 2, 2, COLORS::WHITE);
                    } else {
                        rect_fill(&mut self.pixels, 32+x*2, 64+y*2, 2, 2, COLORS::GRAY);
                    }
                } else {
                    rect_fill(&mut self.pixels, 32+x*2, 64+y*2, 2, 2, col);
                }
            }
        }
    }

    fn color_button(&mut self, x: i32, y: i32, col: COLORS){
        for dy in 1..COLOR_BUTTON_WIDTH-1{
            for dx in 1..COLOR_BUTTON_WIDTH-1{
                set_pix(&mut self.pixels, y + dy, x + dx, col);
            }
        }

        if self.mouse.just_pressed {
            if self.mouse.x != -1 {
                if self.mouse.x >= x && self.mouse.x < x + COLOR_BUTTON_WIDTH && self.mouse.y >= y && self.mouse.y < y + COLOR_BUTTON_WIDTH {
                    self.selected_color = col;
                }
            }
        }

        if self.selected_color == col {
            rect(&mut self.pixels, x, y, COLOR_BUTTON_WIDTH-1, COLOR_BUTTON_WIDTH-1, COLORS::WHITE);
        }
    }

    pub fn update(&mut self) {
        clear(&mut self.pixels, COLORS::GRAY);

        for (i, col) in ALL_COLORS.iter().enumerate(){
            if *col == COLORS::BLANK { continue; }
            self.color_button(16 + (i as i32 % 8) * COLOR_BUTTON_WIDTH, 10 + COLOR_BUTTON_WIDTH * (i > 8) as i32, *col);
        }

        self.draw_canvas();

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
