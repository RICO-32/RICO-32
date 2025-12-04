use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::io::{self, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::render::colors::ALL_COLORS;
use crate::{
    engine::rico::PixelsType,
    render::colors::{COLORS},
};

const FILE_PATH: &str = "assets/sheet.sprt";

pub fn read_sheet(sprites: &mut Vec<PixelsType>) -> io::Result<()> {
    let mut file = File::open(FILE_PATH)?;

    let magic = file.read_u32::<LittleEndian>()?;
    let version = file.read_u16::<LittleEndian>()?;
    let sprites_count = file.read_u16::<LittleEndian>()?;

    if magic != 0x54525053 || version != 1 {
        return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a valid SPRT file"
        ));
    }

    let mut frame_buffer = vec![0u8; sprites_count as usize * 32 * 32];
    file.read_exact(&mut frame_buffer)?;

    for chunk in frame_buffer.chunks(32 * 32){
        let mut sprite = vec![vec![COLORS::BLANK; 32]; 32];
        for i in 0..32{
            for j in 0..32{
                if chunk[i*32+j] > 16 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Not a valid pixel"
                    ));
                }
                
                sprite[i][j] = ALL_COLORS[chunk[i*32 + j] as usize];
            }
        }
        sprites.push(sprite);
    }

    Ok(())
}

pub fn read_image_idx( sprite: &mut PixelsType, idx: usize) -> io::Result<()> {
    let mut file = File::open(FILE_PATH)?;

    let magic = file.read_u32::<LittleEndian>()?;
    let version = file.read_u16::<LittleEndian>()?;
    let sprites_count = file.read_u16::<LittleEndian>()? as usize;

    if magic != 0x54525053 || version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid SPRT file"
        ));
    }

    if idx >= sprites_count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Sprite index out of bounds"
        ));
    }

    let sprite_size = 32 * 32; 
    let header_size = 4 + 2 + 2;

    file.seek(SeekFrom::Start((header_size + idx * sprite_size) as u64))?;

    let mut buffer = vec![0u8; sprite_size];
    file.read_exact(&mut buffer)?;

    for i in 0..32 {
        for j in 0..32 {
            let val = buffer[i * 32 + j];

            if val > 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not a valid pixel"
                ));
            }

            sprite[i][j] = ALL_COLORS[val as usize];
        }
    }

    Ok(())
}

pub fn write_sheet(sprites: &Vec<PixelsType>) -> io::Result<()> {
    let mut file = File::create(FILE_PATH)?;

    let magic: u32 = 0x54525053;
    let version: u16 = 1;
    let sprites_count: u16 = sprites.len() as u16;

    file.write_u32::<LittleEndian>(magic)?;
    file.write_u16::<LittleEndian>(version)?;
    file.write_u16::<LittleEndian>(sprites_count)?;

    for sprite in sprites {
        let flat_bytes: Vec<u8> = sprite
            .iter()
            .flatten()
            .map(|&c| c as u8)
            .collect();

        file.write_all(&flat_bytes)?;
    }

    Ok(())
}
