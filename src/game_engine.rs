use std::collections::{HashMap, HashSet};
use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use image::{ImageBuffer, ImageReader, Rgba};
use mlua::prelude::LuaResult;
use winit::event::VirtualKeyCode;

use crate::log_engine::{LogEngine};
use crate::utils::colors::{color_from_str, str_from_color, COLORS};
use crate::utils::keyboard::key_from_str;
use crate::utils::mouse::MousePress;
use crate::utils::pixels::{circle, clear, draw, print_scr, print_scr_mid, print_scr_mini, rect, rect_fill, set_pix};
use crate::script_engine::ScriptEngine;
use crate::rico_engine::{PixelsType, ScreenEngine, SCREEN_SIZE};

const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u128 = 1000;

pub struct GameEngine{
    pub script_engine: ScriptEngine,
    pub log_engine: LogEngine,
    pub mouse: Rc<RefCell<MousePress>>,
    pub keys_pressed: Rc<RefCell<HashSet<VirtualKeyCode>>>,
    last_time: Rc<RefCell<Instant>>,
    frame_rate: Rc<RefCell<i32>>,
    pixels: Rc<RefCell<PixelsType>>,
    sprites: Rc<RefCell<HashMap<String, ImageBuffer<Rgba<u8>, Vec<u8>>>>>,
}

impl GameEngine{
    pub fn new() -> LuaResult<Self> {
        let last_time = Rc::new(RefCell::new(Instant::now()));
        let script_engine = ScriptEngine::new("scripts")?;

        let mut eng = GameEngine {
            script_engine,
            log_engine: LogEngine::new(last_time.clone()),
            last_time: last_time,
            frame_rate: Rc::new(RefCell::new(BASE_FPS)),
            pixels: Rc::new(RefCell::new(COLORS::pixels())),
            sprites: Rc::new(RefCell::new(HashMap::new())),
            mouse: Rc::new(RefCell::new(MousePress::default())),
            keys_pressed: Rc::new(RefCell::new(HashSet::new())),
        };

        eng.script_engine.boot()?;
        let _ = eng.register_api();
        eng.script_engine.call_start()?;

       Ok(eng)
    }

    //Define all lua API functions here
    pub fn register_api(&mut self) -> LuaResult<()> {
        let lua = &self.script_engine.lua;
        let globals = lua.globals();

        let logs_rc = self.log_engine.logs.clone();
        globals.set(
            "log",
            lua.create_function(move |_, msg: String| {
                let msg = format!("[Log] {}", msg);
                for chunk in msg.as_bytes().chunks(30){
                    logs_rc.borrow_mut().push(String::from_utf8(chunk.to_vec()).unwrap());
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "set_pix",
            lua.create_function(move |_, (x, y, col): (i32, i32, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    set_pix(pix_rc.clone(), y, x, val);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "get_pix",
            lua.create_function(move |_, (x, y): (usize, usize)| {
                if y >= SCREEN_SIZE || x >= SCREEN_SIZE {
                    return Err(mlua::prelude::LuaError::RuntimeError(format!(
                                "Pixel coordinates out of bounds: {}, {}",
                                x, y
                    )));
                }
                Ok(str_from_color(pix_rc.borrow()[y][x]))
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "print_scr",
            lua.create_function(move |_, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(val) = color_from_str(col.as_str()){
                    print_scr(pix_rc.clone(), x, y, val, msg);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "print_scr_mini",
            lua.create_function(move |_, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(val) = color_from_str(col.as_str()){
                    print_scr_mini(pix_rc.clone(), x, y, val, msg);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "print_scr_mid",
            lua.create_function(move |_, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(val) = color_from_str(col.as_str()){
                    print_scr_mid(pix_rc.clone(), x, y, val, msg);
                }
                Ok(())
            })?,
        )?;

        let sprites_rc = self.sprites.clone();
        let pix_rc = self.pixels.clone();
        globals.set(
            "draw",
            lua.create_function(move |_, (x, y, file): (i32, i32, String)| {
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


        let pix_rc = self.pixels.clone();
        globals.set(
            "rectfill",
            lua.create_function(move |_, (x, y, w, h, col): (i32, i32, i32, i32, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    rect_fill(pix_rc.clone(), x, y, w, h, val);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "rect",
            lua.create_function(move |_, (x, y, w, h, col): (i32, i32, i32, i32, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    rect(pix_rc.clone(), x, y, w, h, val);
                }
                Ok(())
            })?,
        )?;

        let pix_rc = self.pixels.clone();
        globals.set(
            "circle",
            lua.create_function(move |_, (x, y, r, col): (i32, i32, i32, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    circle(pix_rc.clone(), x, y, r, val);
                }
                Ok(())
            })?,
        )?;

        let frame_rate_rc = self.frame_rate.clone();
        globals.set(
            "set_frame_rate",
            lua.create_function(move |_, rate: i32| {
                *frame_rate_rc.borrow_mut() = rate;
                Ok(())
            })?,
        )?;

        let mouse_rc = self.mouse.clone();
        globals.set(
            "mouse",
            lua.create_function(move |_, ()| {
                let mut new_mouse = mouse_rc.borrow().clone();
                if mouse_rc.borrow().x == -1 {
                    new_mouse.pressed = false;
                    new_mouse.just_pressed = false;
                }
                Ok(new_mouse)
            })?,
        )?;

        let keys_rc = self.keys_pressed.clone();
        globals.set(
            "key_pressed",
            lua.create_function(move |_, key: String| {
                if let Some(keycode) = key_from_str(key.as_str()){
                    return Ok(keys_rc.borrow().contains(&keycode));
                }
                Ok(false)
            })?,
        )?;

        let pix_rc = self.pixels.clone();
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
            let dt = self.last_time.borrow().elapsed().as_millis();
            *self.last_time.borrow_mut() = now;
            return dt;
        }

        let target_frame_time = time::Duration::from_millis((MILLIS_IN_SEC as f64 / frame_rate as f64) as u64);
        let elapsed_time = self.last_time.borrow().elapsed();

        if elapsed_time < target_frame_time {
            thread::sleep(target_frame_time - elapsed_time);
        }

        let dt = self.last_time.borrow().elapsed().as_millis();
        *self.last_time.borrow_mut() = Instant::now();
        dt
    }
}

impl ScreenEngine for GameEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>>{
        self.pixels.clone()
    }
    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    fn update(&mut self) {
        let dt = self.sync();
        let _ = self.script_engine.call_update(dt);
        if self.mouse.borrow().just_pressed {
            self.mouse.borrow_mut().just_pressed = false;
        };
    }
}
