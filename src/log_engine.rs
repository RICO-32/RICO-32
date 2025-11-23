use std::{cell::RefCell, rc::Rc};

use crate::{rico_engine::{PixelsType, ScreenEngine}, utils::{colors::COLORS, pixels::{clear, print_scr, print_scr_mini}}};

pub struct LogEngine{
    pixels: Rc<RefCell<PixelsType>>,
    pub logs: Rc<RefCell<Vec<String>>>
}

impl LogEngine{
    pub fn new() -> Self{
        LogEngine{
            pixels: Rc::new(RefCell::new(COLORS::pixels())),
            logs: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl ScreenEngine for LogEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>> {
        self.pixels.clone()
    }

    fn update(&mut self) -> mlua::Result<()> {
        clear(self.pixels.clone(), COLORS::GRAY);
        for (i, log) in self.logs.borrow()[self.logs.borrow().len().saturating_sub(23)..].iter().enumerate(){
            print_scr_mini(self.pixels.clone(), 1, 6*i + 8, COLORS::BLACK, log.to_string());
        }
        Ok(())
    }
}
