use std::time::Instant;

use macro_procs::ScreenEngine;

use crate::{engine::rico::{PixelsType, ScreenEngine}, input::mouse::MousePress, render::{colors::COLORS, pixels::{circle, clear, print_scr_mid, rect_fill, set_pix}}, scripting::lua::LogTypes};

#[derive(ScreenEngine)]
pub struct ConsoleEngine{
    pixels: PixelsType,
    pub last_time: Instant,
    pub halted: bool,
    pub mouse: MousePress,
    pub restart: bool,
}

const HALT_BUTTON: (i32, i32, i32, i32) = (50, 2, 13, 9);
const RESTART_BUTTON: (i32, i32, i32, i32) = (66, 2, 13, 9);
const G: COLORS = COLORS::GRAY;
const B: COLORS = COLORS::BLANK;
const RESTART_IMAGE: [[COLORS; 7]; 7] = [
    [B, B, G, G, G, G, B],
    [B, G, G, B, G, G, G],
    [G, G, B, B, B, G, B],
    [G, B, B, B, B, B, B],
    [G, G, B, B, B, B, G],
    [B, G, G, B, B, G, G],
    [B, B, G, G, G, G, B],
];

impl ConsoleEngine{
    pub fn new() -> Self{
        ConsoleEngine{
            pixels: COLORS::pixels(),
            last_time: Instant::now(),
            halted: false,
            mouse: MousePress::default(),
            restart: false
        }
    }

    fn draw_game_control(&mut self) {
        rect_fill(&mut self.pixels, HALT_BUTTON.0, HALT_BUTTON.1, HALT_BUTTON.2, HALT_BUTTON.3, COLORS::SILVER);

        if self.halted {
            circle(&mut self.pixels, 56, 6, 2, COLORS::GREEN);
        } else {
            rect_fill(&mut self.pixels, 54, 4, 5, 5, COLORS::RED);
        }

        rect_fill(&mut self.pixels, RESTART_BUTTON.0, RESTART_BUTTON.1, RESTART_BUTTON.2, RESTART_BUTTON.3, COLORS::SILVER);
        for y in 0..7{
            for x in 0..7{
                set_pix(&mut self.pixels, RESTART_BUTTON.1+1+y, RESTART_BUTTON.0+3+x, RESTART_IMAGE[y as usize][x as usize]);
            }
        }
    }

    fn assess_game_control(&mut self) {
        if self.mouse.just_pressed {
            if self.mouse.x >= HALT_BUTTON.0 && self.mouse.x <= HALT_BUTTON.0 + HALT_BUTTON.2 && self.mouse.y >= HALT_BUTTON.1 && self.mouse.y <= HALT_BUTTON.1 + HALT_BUTTON.3 {
                let curr = self.halted;
                if curr {
                    self.last_time = Instant::now();
                }
                self.halted = !curr;
            }
        }
        if self.restart { self.restart = false };
        if self.mouse.just_pressed {
            if self.mouse.x >= RESTART_BUTTON.0 && self.mouse.x <= RESTART_BUTTON.0 + RESTART_BUTTON.2 && self.mouse.y >= RESTART_BUTTON.1 && self.mouse.y <= RESTART_BUTTON.1 + RESTART_BUTTON.3 {
                self.restart = true;
            }
        }
    }

    pub fn update(&mut self, logs: &Vec<LogTypes>) {
        clear(&mut self.pixels, COLORS::GRAY);
        self.draw_game_control();
        self.assess_game_control();
        for (i, log) in logs[logs.len().saturating_sub(19)..].iter().enumerate(){
            let col = match log {
                LogTypes::Err(_) => COLORS::MAROON,
                LogTypes::Ok(_) => COLORS::BLACK
            };
            print_scr_mid(&mut self.pixels, 1, 6*i as i32 + 2 + 3 * 6, col, log.to_string());
        }
        if self.mouse.just_pressed {
            self.mouse.just_pressed = false;
        };
    }
}
