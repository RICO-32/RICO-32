use std::{cell::RefCell, error::Error, rc::Rc};

use image::ImageReader;

use crate::{bitmap::BITMAP, colors::COLORS, goon_engine::{PixelsType, SCREEN_SIZE}};

pub fn set_pix(pixels: Rc<RefCell<PixelsType>>, y: usize, x: usize, col: COLORS){
    //If the new pixel has 0 alpha, just keep the old guy
    //We don't wanna implement full alpha stuff cause pixel art 
    //This much is fine for images with empty bgs
    if col.3 == 0 {
        return;
    }
    pixels.borrow_mut()[y][x] = col;
}


//Place holder functions
pub fn draw(pixels: Rc<RefCell<PixelsType>>, x: usize, y: usize, file: String) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(format!("assets/{}", file))?.decode()?;
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    println!("size: {} x {}", width, height);

    if width != height || (width != 8 && width != 16 && width != 32) {
        return Ok(());
    }

    for (dx, dy, pixel) in img.enumerate_pixels() {
        let [r, g, b, a] = pixel.0;
        if y+dy as usize >= SCREEN_SIZE as usize || x + dx as usize >= SCREEN_SIZE as usize {
            continue;
        }

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
                if y+dy >= SCREEN_SIZE as usize || x + dx >= SCREEN_SIZE as usize {
                    continue;
                }
                if BITMAP[idx][dy] >> (7-dx) & 1 == 1{
                    set_pix(pixels.clone(), y+dy, x+dx+i*8, col);
                }
            }
        }
    }
}
