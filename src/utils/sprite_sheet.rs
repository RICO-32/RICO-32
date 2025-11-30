use std::fs::File;
use std::io::{self, Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::utils::colors::{ALL_COLORS, COLORS};

fn read_sheet(sprites: &mut Vec<Vec<Vec<COLORS>>>) -> io::Result<()> {
    let mut file = File::open("sheet.sprt")?;

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

    for (s, chunk) in frame_buffer.chunks(32 * 32).enumerate(){
        for i in 0..32{
            for j in 0..32{
                if chunk[i*32+j] > 16 {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Not a valid pixel"
                    ));
                }
                sprites[s][i][j] = ALL_COLORS[chunk[i*32 + j] as usize];
            }
        }
    }

    Ok(())
}

fn write_sheet(sprites: &Vec<Vec<Vec<COLORS>>>) -> io::Result<()> {
    let mut file = File::create("sheet.sprt")?;

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
