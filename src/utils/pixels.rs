use std::{cell::RefCell, error::Error, rc::Rc};

use image::{ImageBuffer, Rgba};

use crate::utils::bitmap::BITMAP4X4;
use crate::utils::{bitmap::BITMAP, colors::COLORS};
use crate::goon_engine::{PixelsType, SCREEN_SIZE};

pub fn set_pix(pixels: Rc<RefCell<PixelsType>>, y: usize, x: usize, col: COLORS){
    //If the new pixel has 0 alpha, just keep the old guy
    //We don't wanna implement full alpha stuff cause pixel art 
    //This much is fine for images with empty bgs
    if col.3 == 0 {
        return;
    }

    if y >= SCREEN_SIZE as usize || x >= SCREEN_SIZE as usize {
        return;
    }

    pixels.borrow_mut()[y][x] = col;
}


//Place holder functions
pub fn draw(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();

    if width != height || (width != 8 && width != 16 && width != 32) {
        return Ok(());
    }

    for (dx, dy, pixel) in img.enumerate_pixels() {
        let [r, g, b, a] = pixel.0;
        set_pix(pixels.clone(), y+dy as usize, x+dx as usize, COLORS(r, g, b, a));
    }

    Ok(())
}

/* Loop over every character and use the 8x8 bitmap
 * Use bitmasking to check which pixels to be set 
 */
pub fn print_scr(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, col: COLORS, msg: String){
    for i in 0..msg.len(){
        let c = msg.as_bytes().iter().nth(i).unwrap();
        let mut idx: usize = (*c).into();
        idx -= 32;
        if idx >= BITMAP.len() {
            idx = 0;
        }

        for dx in 0..8{
            for dy in 0..8{
                if BITMAP[idx][dy] >> (7-dx) & 1 == 1{
                    set_pix(pixels.clone(), y+dy, x+dx+i*9, col);
                }
            }
        }
    }
}

pub fn print_scr_mini(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, col: COLORS, msg: String){
    for i in 0..msg.len(){
        let c = msg.as_bytes().iter().nth(i).unwrap();
        let mut idx: usize = (*c).into();
        let orig_idx: usize = (*c).into();
        idx -= 32;
        idx /= 2;
        idx *= 4;

        if idx >= BITMAP4X4.len() {
            idx = 0;
        }

        for dx in 0..4{
            for dy in 0..4{
                if (BITMAP4X4[idx+dx] >> (3-dy)) >> ((orig_idx & 1) * 4) & 1 == 1{
                    set_pix(pixels.clone(), y-dy+3, x+dx+i*5, col);
                }
            }
        }
    }
}

pub fn rect_fill(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, w: usize, h: usize, col: COLORS){
    for j in x..=x+w as usize{
        for i in y..=y+h as usize{
            set_pix(pixels.clone(), i, j, col);
        }
    }
}

pub fn rect(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, w: usize, h: usize, col: COLORS){
    for i in x..=x+w as usize{
        set_pix(pixels.clone(), y, i, col);
        set_pix(pixels.clone(), y+h, i, col);
    }

    for i in y..=y+h as usize{
        set_pix(pixels.clone(), i, x, col);
        set_pix(pixels.clone(), i, x+w, col);
    }
}

pub fn clear(pixels: Rc<RefCell<PixelsType>>, col: COLORS){
    for y in 0..SCREEN_SIZE as usize{
        for x in 0..SCREEN_SIZE as usize{
            set_pix(pixels.clone(), y, x, col);
        }
    }
}
