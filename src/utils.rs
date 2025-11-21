use std::{cell::RefCell, rc::Rc};

use crate::{colors::COLORS, goon_engine::PixelsType};

pub fn set_pix(pixels: Rc<RefCell<PixelsType>>, y: usize, x: usize, col: COLORS){
    //If the new pixel has 0 alpha, just keep the old guy
    //We don't wanna implement full alpha stuff cause pixel art 
    //This much is fine for images with empty bgs
    if col.3 == 0 {
        return;
    }
    pixels.borrow_mut()[y][x] = col;
}

