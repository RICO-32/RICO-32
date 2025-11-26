use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use image::{ImageBuffer, ImageReader, Rgba};
use mlua::UserData;

use crate::game_engine::BASE_FPS;
use crate::utils::colors::{color_from_str, str_from_color, COLORS};
use crate::utils::keyboard::{key_from_str, Keyboard};
use crate::utils::mouse::MousePress;
use crate::utils::pixels::{circle, clear, draw, print_scr, print_scr_mid, print_scr_mini, rect, rect_fill, set_pix};
use crate::rico_engine::{PixelsType, SCREEN_SIZE};

pub struct LuaAPI{
    pub mouse: MousePress,
    pub keyboard: Keyboard,
    pub frame_rate: i32,
    pub pixels: PixelsType,
    pub sprites: HashMap<String, ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub logs: Vec<String>
}

impl LuaAPI {
    pub fn default() -> Self {
        LuaAPI {
            frame_rate: BASE_FPS,
            pixels: COLORS::pixels(),
            logs: Vec::new(),
            sprites: HashMap::new(),
            mouse: MousePress::default(),
            keyboard: Keyboard::default()
        }
    }
}

#[derive(Clone)]
pub struct LuaAPIHandle(pub Rc<RefCell<LuaAPI>>);

impl UserData for LuaAPIHandle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("log", move |_, this, msg: String| {
                let msg = format!("[Log] {}", msg);
                for chunk in msg.as_bytes().chunks(30){
                    this.0.borrow_mut().logs.push(String::from_utf8(chunk.to_vec()).unwrap());
                }
                Ok(())
        });

        methods.add_method("set_pix", move |_, this, (x, y, col): (i32, i32, String)| {
            if let Some(val) = color_from_str(&col.to_string()){
                set_pix(&mut this.0.borrow_mut().pixels, y, x, val);
            }
            Ok(())
        });

        methods.add_method("get_pix", |_, this, (x, y): (usize, usize)| {
            if x >= SCREEN_SIZE || y >= SCREEN_SIZE {
                return Err(mlua::Error::RuntimeError(format!(
                            "Pixel out of bounds: {}, {}", x, y
                )));
            }
            Ok(str_from_color(this.0.borrow().pixels[y][x]))
        });

        methods.add_method_mut("print_scr",
            |_, this, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(c) = color_from_str(&col) {
                    print_scr(&mut this.0.borrow_mut().pixels, x, y, c, msg);
                }
                Ok(())
            }
        );

        methods.add_method_mut("print_scr_mini",
            |_, this, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(c) = color_from_str(&col) {
                    print_scr_mini(&mut this.0.borrow_mut().pixels, x, y, c, msg);
                }
                Ok(())
            }
        );

        methods.add_method_mut("print_scr_mid",
            |_, this, (x, y, col, msg): (i32, i32, String, String)| {
                if let Some(c) = color_from_str(&col) {
                    print_scr_mid(&mut this.0.borrow_mut().pixels, x, y, c, msg);
                }
                Ok(())
            }
        );

        methods.add_method_mut("draw",
            |_, this, (x, y, file): (i32, i32, String)| {
                let mut eng = this.0.borrow_mut(); // mutable borrow once
                let img = if let Some(i) = eng.sprites.get(&file) {
                    i.clone()
                } else {
                    let loaded = ImageReader::open(format!("assets/{}", file))
                        .map_err(mlua::Error::external)?
                        .decode()
                        .map_err(mlua::Error::external)?
                        .to_rgba8();

                    eng.sprites.insert(file.clone(), loaded);
                    eng.sprites.get(&file).unwrap().clone()
                };

                draw(&mut eng.pixels, x, y, &img)
                    .map_err(mlua::Error::external)
            }
        );


        methods.add_method_mut("rectfill",
            |_, this, (x, y, w, h, col): (i32, i32, i32, i32, String)| {
                if let Some(c) = color_from_str(&col) {
                    rect_fill(&mut this.0.borrow_mut().pixels, x, y, w, h, c);
                }
                Ok(())
            }
        );

        methods.add_method_mut("rect",
            |_, this, (x, y, w, h, col): (i32, i32, i32, i32, String)| {
                if let Some(c) = color_from_str(&col) {
                    rect(&mut this.0.borrow_mut().pixels, x, y, w, h, c);
                }
                Ok(())
            }
        );

        methods.add_method_mut("circle",
            |_, this, (x, y, r, col): (i32, i32, i32, String)| {
                if let Some(c) = color_from_str(&col) {
                    circle(&mut this.0.borrow_mut().pixels, x, y, r, c);
                }
                Ok(())
            }
        );

        methods.add_method_mut("clear",
            |_, this, col: String| {
                if let Some(c) = color_from_str(&col) {
                    clear(&mut this.0.borrow_mut().pixels, c);
                }
                Ok(())
            }
        );

        methods.add_method_mut("set_frame_rate",
            |_, this, rate: i32| {
                this.0.borrow_mut().frame_rate = rate;
                Ok(())
            }
        );

        methods.add_method("mouse", |_, this, ()| {
            let mut m = this.0.borrow().mouse.clone();
            if this.0.borrow().mouse.x == -1 {
                m.pressed = false;
                m.just_pressed = false;
            }
            Ok(m)
        });

        methods.add_method("key_pressed",
            |_, this, key: String| {
                if let Some(kc) = key_from_str(&key) {
                    return Ok(this.0.borrow().keyboard.keys_pressed.contains(&kc));
                }
                Ok(false)
            }
        );

        methods.add_method("key_just_pressed",
            |_, this, key: String| {
                if let Some(kc) = key_from_str(&key) {
                    return Ok(this.0.borrow().keyboard.keys_just_pressed.contains(&kc));
                }
                Ok(false)
            }
        );
    }
}

