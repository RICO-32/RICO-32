use std::collections::{HashMap};
use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use image::{ImageBuffer, ImageReader, Rgba};
use mlua::prelude::LuaResult;

use crate::colors::{color_from_str, str_from_color, COLORS};
use crate::script_engine::ScriptEngine;
use crate::goon_engine::{PixelsType, ScreenEngine, SCREEN_SIZE};
use crate::utils::{clear, draw, print_scr, set_pix};

const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u128 = 1000;

pub struct GameEngine{
    pub script_engine: ScriptEngine,
    last_time: Instant,
    frame_rate: Rc<RefCell<i32>>,
    pixels: Rc<RefCell<PixelsType>>,
    sprites: Rc<RefCell<HashMap<String, ImageBuffer<Rgba<u8>, Vec<u8>>>>>
}

impl GameEngine{
    pub fn new() -> LuaResult<Self> {
        let frame_rate = Rc::new(RefCell::new(BASE_FPS));
        let script_engine = ScriptEngine::new("scripts")?;

        let mut eng = GameEngine {
            script_engine,
            last_time: Instant::now(),
            frame_rate: frame_rate.clone(),
            pixels: Rc::new(RefCell::new(COLORS::pixels())),
            sprites: Rc::new(RefCell::new(HashMap::new())),
        };

        eng.script_engine.boot()?;
        let _ = eng.register_api();
        eng.script_engine.call_start()?;

       Ok(eng)
    }

    //Define all lua API functions here
    pub fn register_api(&mut self) -> LuaResult<()> {
        let eng_rc = Rc::from(RefCell::from(self));
        let lua = &eng_rc.borrow().script_engine.lua;
        let globals = lua.globals();

        globals.set(
            "log",
            lua.create_function(move |_, msg: String| {
                println!("{}", format!("[Lua] {}", msg));
                Ok(())
            })?,
        )?;

        let pix_rc = eng_rc.clone().borrow().pixels.clone();
        globals.set(
            "set_pix",
            lua.create_function(move |_, (x, y, col): (usize, usize, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    if y >= SCREEN_SIZE as usize || x >= SCREEN_SIZE as usize{
                        return Err(mlua::prelude::LuaError::RuntimeError(format!(
                                    "Pixel coordinates out of bounds: {}, {}",
                                    x, y
                        )));
                    }
                    set_pix(pix_rc.clone(), y, x, val);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = eng_rc.clone().borrow().pixels.clone();
        globals.set(
            "get_pix",
            lua.create_function(move |_, (x, y): (usize, usize)| {
                if y >= SCREEN_SIZE as usize || x >= SCREEN_SIZE as usize{
                    return Err(mlua::prelude::LuaError::RuntimeError(format!(
                                "Pixel coordinates out of bounds: {}, {}",
                                x, y
                    )));
                }
                Ok(str_from_color(pix_rc.borrow()[y][x]))
            })?,
        )?;

        let pix_rc = eng_rc.borrow().pixels.clone();
        globals.set(
            "print_scr",
            lua.create_function(move |_, (x, y, col, msg): (usize, usize, String, String)| {
                if let Some(val) = color_from_str(col.as_str()){
                    print_scr(pix_rc.clone(), x, y, val, msg);
                }
                Ok(())
            })?,
        )?;

        let sprites_rc = eng_rc.borrow().sprites.clone();
        let pix_rc = eng_rc.borrow().pixels.clone();
        globals.set(
            "draw",
            lua.create_function(move |_, (x, y, file): (usize, usize, String)| {
                let mut sprites = sprites_rc.borrow_mut();
                let img = match sprites.get(&file) {
                    Some(img) => img,
                    None => {
                        let loaded = ImageReader::open(format!("assets/{}", file))
                            .map_err(mlua::Error::external)?
                            .decode()
                            .map_err(mlua::Error::external)?
                            .to_rgba8();

                        sprites.insert(file.clone(), loaded);
                        sprites.get(&file).unwrap()
                    }
                };

                draw(pix_rc.clone(), x, y, img).map_err(mlua::Error::external)
            })?,
        )?;

        let frame_rate_rc = eng_rc.borrow().frame_rate.clone();
        globals.set(
            "set_frame_rate",
            lua.create_function(move |_, rate: i32| {
                *frame_rate_rc.borrow_mut() = rate;
                Ok(())
            })?,
        )?;

        let pix_rc = eng_rc.borrow().pixels.clone();
        globals.set(
            "clear",
            lua.create_function(move |_, col: String| {
                if let Some(val) = color_from_str(col.as_str()){
                    clear(pix_rc.clone(), val);
                }
                Ok(())
            })?,
        )?;

        Ok(())
    }

    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&mut self) -> u128 {
        let frame_rate = *self.frame_rate.borrow();
        if frame_rate <= 0 {
            let now = Instant::now();
            let dt = self.last_time.elapsed().as_millis();
            self.last_time = now;
            return dt;
        }

        let target_frame_time = time::Duration::from_millis((MILLIS_IN_SEC as f64 / frame_rate as f64) as u64);
        let elapsed_time = self.last_time.elapsed();

        if elapsed_time < target_frame_time {
            thread::sleep(target_frame_time - elapsed_time);
        }

        let dt = self.last_time.elapsed().as_millis();
        self.last_time = Instant::now();
        dt
    }
}

impl ScreenEngine for GameEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>>{
        self.pixels.clone()
    }
    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    fn update(&mut self) -> LuaResult<()> {
        let dt = self.sync();
        self.script_engine.call_update(dt)
    }
}
