use std::{cell::RefCell, rc::Rc};

use crate::{rico_engine::{PixelsType, ScreenEngine}, utils::{colors::COLORS, mouse::MousePress, pixels::{circle, clear, print_scr_mid, rect_fill}}};

pub struct LogEngine{
    pixels: Rc<RefCell<PixelsType>>,
    pub logs: Rc<RefCell<Vec<String>>>,
    pub halted: Rc<RefCell<bool>>,
    pub mouse: Rc<RefCell<MousePress>>,
    pub restart: bool
}

const HALT_BUTTON: (usize, usize, usize, usize) = (50, 2, 12, 8);
const RESTART_BUTTON: (usize, usize, usize, usize) = (66, 2, 12, 8);

impl LogEngine{
    pub fn new() -> Self{
        LogEngine{
            pixels: Rc::new(RefCell::new(COLORS::pixels())),
            logs: Rc::new(RefCell::new(Vec::new())),
            halted: Rc::new(RefCell::new(false)),
            mouse: Rc::new(RefCell::new(MousePress::default())),
            restart: false
        }
    }

    fn draw_game_control(&mut self) {
        rect_fill(self.pixels.clone(), HALT_BUTTON.0, HALT_BUTTON.1, HALT_BUTTON.2, HALT_BUTTON.3, COLORS::SILVER);
        rect_fill(self.pixels.clone(), RESTART_BUTTON.0, RESTART_BUTTON.1, RESTART_BUTTON.2, RESTART_BUTTON.3, COLORS::SILVER);

        if *self.halted.borrow() {
            circle(self.pixels.clone(), 56, 6, 2, COLORS::GREEN);
        } else {
            rect_fill(self.pixels.clone(), 54, 4, 4, 4, COLORS::RED);
        }
    }

    fn assess_game_control(&mut self) {
        let mouse = self.mouse.borrow();
        if mouse.just_pressed {
            if mouse.x as usize >= HALT_BUTTON.0 && mouse.x as usize <= HALT_BUTTON.0 + HALT_BUTTON.2 && mouse.y as usize >= HALT_BUTTON.1 && mouse.y as usize <= HALT_BUTTON.1 + HALT_BUTTON.3 {
                let curr = *self.halted.borrow_mut();
                *self.halted.borrow_mut() = !curr;
            }
        }
        if self.restart { self.restart = false };
        if mouse.just_pressed {
            if mouse.x as usize >= RESTART_BUTTON.0 && mouse.x as usize <= RESTART_BUTTON.0 + RESTART_BUTTON.2 && mouse.y as usize >= RESTART_BUTTON.1 && mouse.y as usize <= RESTART_BUTTON.1 + RESTART_BUTTON.3 {
                self.restart = true;
            }
        }
    }
}

impl ScreenEngine for LogEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>> {
        self.pixels.clone()
    }

    fn update(&mut self) {
        clear(self.pixels.clone(), COLORS::GRAY);
        self.draw_game_control();
        self.assess_game_control();
        for (i, log) in self.logs.borrow()[self.logs.borrow().len().saturating_sub(20)..].iter().enumerate(){
            print_scr_mid(self.pixels.clone(), 1, 6*i + 2 + 3 * 6, COLORS::BLACK, log.to_string());
        }
        if self.mouse.borrow().just_pressed {
            self.mouse.borrow_mut().just_pressed = false;
        };
    }
}
