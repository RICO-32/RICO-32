use std::{collections::HashMap, error::Error, fs, path::Path};

use crate::{
    engine::{rico::PixelsType, sprite::SPRITE_SIZE},
    render::colors::Colors,
};
use bincode::{Decode, Encode};
use walkdir::WalkDir;

#[derive(Encode, Decode, Debug, Clone)]
pub struct Cartridge {
    pub sprite_sheet: Vec<PixelsType>,
    pub scripts: HashMap<String, String>,
}

pub const PATH: &str = "r32/";
const BIN_PATH: &str = "main.r32";

const HELLO_WORLD: &str = "
function start()
    rico:log(\"Welcome to RICO-32!\")
    rico:set_frame_rate(60)
end

function update(dt)
    rico:clear(\"BLACK\")
    rico:print_scr(10, 10, \"WHITE\", \"Hello, World!\")
    
    local mouse = rico:mouse()
    if mouse.pressed then
        rico:circle(mouse.x, mouse.y, 5, \"RED\")
    end
end";

impl Default for Cartridge {
    fn default() -> Self {
        let mut scripts = HashMap::new();
        scripts.insert("main.lua".to_string(), HELLO_WORLD.to_string());
        Cartridge {
            sprite_sheet: vec![vec![vec![Colors::Blank; SPRITE_SIZE]; SPRITE_SIZE]; 60],
            scripts,
        }
    }
}

pub fn get_cart() -> Result<Cartridge, Box<dyn Error>> {
    match fs::read(BIN_PATH) {
        Ok(data) => {
            let (cart, _len): (Cartridge, usize) =
                bincode::decode_from_slice(&data, bincode::config::standard())?;
            Ok(cart)
        }
        Err(_) => {
            let cart = Cartridge::default();
            let encoded = bincode::encode_to_vec(&cart, bincode::config::standard())?;
            fs::write(BIN_PATH, encoded)?;
            Ok(cart)
        }
    }
}

pub fn load_cartridge() -> Result<Cartridge, Box<dyn Error>> {
    let cart = get_cart()?;

    if Path::new(PATH).exists() {
        fs::remove_dir_all(PATH)?;
    }
    for (file, content) in &cart.scripts {
        let f_path = PATH.to_owned() + file;
        if let Some(parent) = Path::new(&f_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(f_path, content)?;
    }

    Ok(cart)
}

pub fn update_sprites(sprite_sheet: &[PixelsType]) -> Result<(), Box<dyn Error>> {
    let mut cart = get_cart()?;
    cart.sprite_sheet = sprite_sheet.to_vec();

    let encoded: Vec<u8> = bincode::encode_to_vec(cart, bincode::config::standard())?;
    fs::write(BIN_PATH, &encoded)?;
    Ok(())
}

pub fn update_scripts() -> Result<(), Box<dyn Error>> {
    let mut cart = get_cart()?;
    let mut scripts = HashMap::new();

    for entry in WalkDir::new(PATH)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.file_name().to_str().unwrap().ends_with(".lua"))
    {
        let path = entry.path();
        let rel = path.strip_prefix(PATH).unwrap().to_string_lossy().to_string();
        let contents = fs::read_to_string(path)?;
        scripts.insert(rel, contents);
    }

    cart.scripts = scripts;

    let encoded: Vec<u8> = bincode::encode_to_vec(cart, bincode::config::standard())?;
    fs::write(BIN_PATH, &encoded)?;
    Ok(())
}
