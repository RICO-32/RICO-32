use std::error::Error;

use image::{ImageBuffer, Rgba};

use crate::sprite_engine::{Tools, Utils, BUTTON_WIDTH};
use crate::utils::bitmap::{BITMAP4X4, BITMAP4X6};
use crate::utils::{bitmap::BITMAP, colors::COLORS};
use crate::rico_engine::{PixelsType, SCREEN_SIZE};

pub fn set_pix(pixels: &mut PixelsType, y: i32, x: i32, col: COLORS){
    //If the new pixel has 0 alpha, just keep the old guy
    //We don't wanna implement full alpha stuff cause pixel art 
    //This much is fine for images with empty bgs
    if col.3 == 0 {
        return;
    }

    if y < 0 || x < 0 || y >= pixels.len() as i32 || x >= pixels[0].len() as i32 {
        return;
    }

    pixels[y as usize][x as usize] = col;
}


//Place holder functions
pub fn draw(pixels: &mut PixelsType, x: i32, y: i32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();

    if width as usize >= SCREEN_SIZE || height as usize >= SCREEN_SIZE{
        return Ok(());
    }

    for (dx, dy, pixel) in img.enumerate_pixels() {
        let [r, g, b, a] = pixel.0;
        set_pix(pixels, y+dy as i32, x+dx as i32, COLORS(r, g, b, a));
    }

    Ok(())
}

/* Loop over every character and use the 8x8 bitmap
 * Use bitmasking to check which pixels to be set 
 */
pub fn print_scr(pixels: &mut PixelsType, x: i32, y: i32, col: COLORS, msg: String){
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
                    set_pix(pixels, y+dy as i32, x+dx as i32+i as i32*8, col);
                }
            }
        }
    }
}

pub fn print_scr_mini(pixels: &mut PixelsType, x: i32, y: i32, col: COLORS, msg: String){
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
                    set_pix(pixels, ((y as i32)-(dy as i32))+3, x+dx as i32+i as i32*5, col);
                }
            }
        }
    }
}

pub fn print_scr_mid(pixels: &mut PixelsType, x: i32, y: i32, col: COLORS, msg: String){
    for i in 0..msg.len(){
        let c = msg.as_bytes().iter().nth(i).unwrap();
        let mut idx: usize = (*c).into();

        if idx >= BITMAP4X6.len() {
            idx = 32;
        }

        for dx in 0..4{
            for dy in 0..6{
                //println!("{} {}", idx, BI
                if BITMAP4X6[idx][dy] >> (3-dx) & 1 == 1{
                    set_pix(pixels, y+dy as i32, x+dx as i32+i as i32*4, col);
                }
            }
        }
    }
}

pub fn rect_fill(pixels: &mut PixelsType, x: i32, y: i32, w: i32, h: i32, col: COLORS){
    for j in x..x+w{
        for i in y..y+h{
            set_pix(pixels, i, j, col);
        }
    }
}

pub fn rect(pixels: &mut PixelsType, x: i32, y: i32, w: i32, h: i32, col: COLORS){
    for i in x..x+w{
        set_pix(pixels, y, i, col);
        set_pix(pixels, y+h, i, col);
    }

    for i in y..y+h{
        set_pix(pixels, i, x, col);
        set_pix(pixels, i, x+w, col);
    }

    set_pix(pixels, y+h, x+w, col);
}

pub fn circle(pixels: &mut PixelsType, cx: i32, cy: i32, r: i32, col: COLORS){
    let r2 = r * r;
    for x in cx-r..=cx+r{
        for y in cy-r..=cy+r{
            let dx = x - cx;
            let dy = y - cy;
            if dx*dx + dy*dy <= r2 {
                set_pix(pixels, y, x, col);
            }
        }
    }
}

pub fn clear(pixels: &mut PixelsType, col: COLORS){
    let height = pixels.len() as i32;
    let width = pixels[0].len() as i32;
    for y in 0..height{
        for x in 0..width{
            set_pix(pixels, y, x, col);
        }
    }
}

pub fn image_from_tool(tool: Tools) -> [[COLORS; BUTTON_WIDTH as usize - 2]; BUTTON_WIDTH as usize - 2] {
    let ye = COLORS::YELLOW;
    let br = COLORS::BROWN;
    let bl = COLORS::BLANK;
    let re = COLORS::RED;
    let db = COLORS::BLUE;
    let si = COLORS::SILVER;
    let gr = COLORS::GRAY;
    let pi = COLORS::PINK;
    match tool {
        Tools::Pencil => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Tools::Fill => {
            [
                [bl, bl, bl, bl, si, gr, gr, bl, bl, bl],
                [bl, bl, bl, si, si, gr, gr, bl, bl, bl],
                [bl, db, db, si, gr, gr, gr, si, bl, bl],
                [db, db, si, si, gr, gr, si, si, si, bl],
                [db, si, si, si, gr, gr, si, si, si, si],
                [db, si, si, gr, gr, gr, si, si, si, si],
                [db, si, si, si, si, si, si, si, si, si],
                [db, bl, si, si, si, si, si, si, si, si],
                [db, bl, bl, si, si, si, si, si, si, si],
                [bl, bl, bl, bl, bl, si, si, si, si, bl],
            ]
        },
        Tools::Eraser => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, re, re, bl, bl, bl],
                [bl, bl, bl, bl, re, pi, re, re, bl, bl],
                [bl, bl, bl, re, re, re, pi, re, re, bl],
                [bl, bl, re, re, re, re, re, pi, re, bl],
                [bl, re, pi, re, re, re, re, re, bl, bl],
                [bl, re, re, pi, re, re, re, bl, bl, bl],
                [bl, bl, re, re, pi, re, bl, bl, bl, bl],
                [bl, bl, bl, re, re, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Tools::Select => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, si, bl, si, si, bl, si, si, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, si, si, bl, si, si, bl, si, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
    }
}

pub fn image_from_util(util: Utils) -> [[COLORS; BUTTON_WIDTH as usize - 2]; BUTTON_WIDTH as usize - 2] {
    let ye = COLORS::YELLOW;
    let br = COLORS::BROWN;
    let bl = COLORS::BLANK;
    let re = COLORS::RED;
    let db = COLORS::BLUE;
    let si = COLORS::SILVER;
    let gr = COLORS::GRAY;
    let pi = COLORS::PINK;
    match util {
        Utils::FlipHor => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Utils::FlipVert => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Utils::Clear => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Utils::Save => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
    }
}

