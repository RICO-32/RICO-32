use std::{cell::RefCell, rc::Rc};

use crate::{rico_engine::{PixelsType, ScreenEngine}, utils::{colors::COLORS, pixels::{circle, clear, print_scr_mid, rect_fill}}};

pub struct LogEngine{
    pixels: Rc<RefCell<PixelsType>>,
    pub logs: Rc<RefCell<Vec<String>>>,
    pub halted: Rc<RefCell<bool>>
}

impl LogEngine{
    pub fn new() -> Self{
        LogEngine{
            pixels: Rc::new(RefCell::new(COLORS::pixels())),
            logs: Rc::new(RefCell::new(Vec::new())),
            halted: Rc::new(RefCell::new(false)),
        }
    }

    fn draw_game_control(&mut self) {
        rect_fill(self.pixels.clone(), 50, 2, 12, 8, COLORS::SILVER);
        rect_fill(self.pixels.clone(), 66, 2, 12, 8, COLORS::SILVER);

        if *self.halted.borrow() {
            circle(self.pixels.clone(), 56, 6, 2, COLORS::GREEN);
        } else {
            rect_fill(self.pixels.clone(), 54, 4, 4, 4, COLORS::RED);
        }
    }
}

impl ScreenEngine for LogEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>> {
        self.pixels.clone()
    }

    fn update(&mut self) -> mlua::Result<()> {
        clear(self.pixels.clone(), COLORS::GRAY);
        self.draw_game_control();
        for (i, log) in self.logs.borrow()[self.logs.borrow().len().saturating_sub(20)..].iter().enumerate(){
            print_scr_mid(self.pixels.clone(), 1, 6*i + 2 + 3 * 6, COLORS::BLACK, log.to_string());
        }
        Ok(())
    }
}
