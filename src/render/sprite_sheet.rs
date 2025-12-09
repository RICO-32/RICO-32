use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{self, Write};
use std::io::{Read, Seek, SeekFrom};

use crate::engine::sprite::SPRITE_SIZE;
use crate::render::colors::ALL_COLORS;
use crate::{engine::rico::PixelsType, render::colors::Colors};

const FILE_PATH: &str = "assets/sheet.sprt";

//SPRT in hex
const MAGIC: u32 = 0x54525053;

//IMPORTANT: Make sure to use little endian everywhere for consistency

pub fn read_sheet(sprites: &mut Vec<PixelsType>) -> io::Result<()> {
    let mut file = File::open(FILE_PATH)?;

    let magic = file.read_u32::<LittleEndian>()?;
    let version = file.read_u16::<LittleEndian>()?;
    let sprites_count = file.read_u16::<LittleEndian>()?;

    if magic != MAGIC || version != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid SPRT file"));
    }

    let mut frame_buffer = vec![0u8; sprites_count as usize * SPRITE_SIZE * SPRITE_SIZE];
    file.read_exact(&mut frame_buffer)?;

    for chunk in frame_buffer.chunks(SPRITE_SIZE * SPRITE_SIZE) {
        let mut sprite = Colors::pixels(SPRITE_SIZE, SPRITE_SIZE);
        for i in 0..SPRITE_SIZE {
            for j in 0..SPRITE_SIZE {
                //If the integer is not one of the valid colors in rico
                if chunk[i * SPRITE_SIZE + j] > 16 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid pixel"));
                }

                sprite[i][j] = ALL_COLORS[chunk[i * SPRITE_SIZE + j] as usize];
            }
        }
        sprites.push(sprite);
    }

    Ok(())
}

//Only for specific idx so we don't load the whole thing in
pub fn read_image_idx(sprite: &mut PixelsType, idx: usize) -> io::Result<()> {
    let mut file = File::open(FILE_PATH)?;

    let magic = file.read_u32::<LittleEndian>()?;
    let version = file.read_u16::<LittleEndian>()?;
    let sprites_count = file.read_u16::<LittleEndian>()? as usize;

    if magic != MAGIC || version != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid SPRT file"));
    }

    if idx >= sprites_count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Sprite index {} out of bounds", idx),
        ));
    }

    let sprite_size = SPRITE_SIZE * SPRITE_SIZE;
    let header_size = 4 + 2 + 2;

    file.seek(SeekFrom::Start((header_size + idx * sprite_size) as u64))?;

    let mut buffer = vec![0u8; sprite_size];
    file.read_exact(&mut buffer)?;

    for i in 0..SPRITE_SIZE {
        for j in 0..SPRITE_SIZE {
            let val = buffer[i * SPRITE_SIZE + j];

            if val > 16 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid pixel"));
            }

            sprite[i][j] = ALL_COLORS[val as usize];
        }
    }

    Ok(())
}

pub fn write_sheet(sprites: &Vec<PixelsType>) -> io::Result<()> {
    let mut file = File::create(FILE_PATH)?;

    let version: u16 = 1;
    let sprites_count: u16 = sprites.len() as u16;

    file.write_u32::<LittleEndian>(MAGIC)?;
    file.write_u16::<LittleEndian>(version)?;
    file.write_u16::<LittleEndian>(sprites_count)?;

    //Literally just copying over bytes as u8s
    for sprite in sprites {
        let flat_bytes: Vec<u8> = sprite.iter().flatten().map(|&c| c as u8).collect();
        file.write_all(&flat_bytes)?;
    }

    Ok(())
}
