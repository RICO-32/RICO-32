use mlua::prelude::*;
use mlua::StdLib;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs};

use crate::colors::color_from_str;
use crate::colors::str_from_color;
use crate::game_engine::Commands;
use crate::goon_engine::PixelsType;
use crate::goon_engine::SCREEN_SIZE;

pub struct ScriptEngine {
    pub lua: Lua,
    scripts_dir: String,
}

impl ScriptEngine {
    /* Will also handle rebooting
     * Redo the engine initialization whenever restarting the game in the engine
     * Call new, boot, and call_start
     */
    pub fn new(scripts_dir: &str) -> LuaResult<Self> {
        let options = LuaOptions::new();
        let lua = Lua::new_with(StdLib::ALL_SAFE, options).expect("Could not load lua state");

        let engine = ScriptEngine {
            lua,
            scripts_dir: String::from(scripts_dir),
        };

        engine.register_loader()?;

        Ok(engine)
    }


    pub fn register_api_in_place(&mut self, pixels: Rc<RefCell<PixelsType>>) -> LuaResult<()> {
        let globals = self.lua.globals();

        let pix_rc = pixels.clone();
        globals.set(
            "set_pix",
            self.lua.create_function(move |_, (x, y, col): (usize, usize, String)| {
                if let Some(val) = color_from_str(&col.to_string()){
                    if y >= SCREEN_SIZE as usize || x >= SCREEN_SIZE as usize{
                        return Err(LuaError::RuntimeError(format!(
                                    "Pixel coordinates out of bounds: {}, {}",
                                    x, y
                        )));
                    }
                    pix_rc.borrow_mut()[y][x] = val;
                }
                Ok(())
            })?,
        )?;

        let pix_rc = pixels.clone();
        globals.set(
            "get_pix",
            self.lua.create_function(move |_, (x, y): (usize, usize)| {
                if y >= SCREEN_SIZE as usize || x >= SCREEN_SIZE as usize{
                    return Err(LuaError::RuntimeError(format!(
                                "Pixel coordinates out of bounds: {}, {}",
                                x, y
                    )));
                }
                Ok(str_from_color(pix_rc.borrow()[y][x]))
            })?,
        )?;

        Ok(())
    }

    //Define all lua API functions here that go into the commands vec in game engine
    pub fn register_api_commands(&mut self, commands: Rc<RefCell<Vec<Commands>>>) -> LuaResult<()> {
        let globals = self.lua.globals();

        let com_rc = commands.clone();
        globals.set(
            "log",
            self.lua.create_function(move |_, msg: String| {
                com_rc.borrow_mut().push(Commands::Log(msg.clone()));
                Ok(())
            })?,
        )?;

        //Mutex bs to deal with lua functions being global, avoids self going out of scope
        let com_rc = commands.clone();
        globals.set(
            "set_frame_rate",
            self.lua.create_function(move |_, rate: i32| {
                println!("{}", rate);
                com_rc.borrow_mut().push(Commands::SetFrameRate(rate));
                Ok(())
            })?,
        )?;

        let com_rc = commands.clone();
        globals.set(
            "draw",
            self.lua.create_function(move |_, (x, y, msg): (usize, usize, String)| {
                com_rc.borrow_mut().push(Commands::Draw(x, y, msg));
                Ok(())
            })?,
        )?;

        let com_rc = commands.clone();
        globals.set(
            "button",
            self.lua.create_function(move |_, (x, y, msg): (usize, usize, String)| {
                com_rc.borrow_mut().push(Commands::Button(x, y, msg));
                Ok(())
            })?,
        )?;

        let com_rc = commands.clone();
        globals.set(
            "print_scr",
            self.lua.create_function(move |_, (x, y, col, msg): (usize, usize, String, String)| {
                if let Some(val) = color_from_str(col.as_str()){
                    com_rc.borrow_mut().push(Commands::PrintScr(x, y, val, msg));
                }
                Ok(())
            })?,
        )?;

        Ok(())
    }

    /* Only way I could think to allow modularization
     * Just a wrapper to load in lua scripts and insert into main
     * Keeps Lua API the same 
     */
    fn register_loader(&self) -> LuaResult<()> {
        let scripts = self.scripts_dir.clone();
        let lua = &self.lua;

        let loader = lua.create_function(move |lua, module: String| {
            let path1 = format!("{}/{}.lua", scripts, module.replace(".", "/"));
            let path2 = format!("{}/{}/init.lua", scripts, module.replace(".", "/"));

            let code = fs::read_to_string(&path1)
                .or_else(|_| fs::read_to_string(&path2))
                .or_else(|_| Ok::<String, String>(String::from("")))
                .unwrap();

            let func = lua.load(&code).into_function()?;
            Ok(func)
        })?;

        //Simply overwrite the inbuilt package loader with ours
        let globals = self.lua.globals();
        let package: LuaTable = globals.get("package")?;
        let searchers: LuaTable = package.get("searchers")?;
        searchers.raw_insert(1, loader)?;

        Ok(())
    }

    //Execs the main lua file, mainly for global stuff
    pub fn boot(&self) -> LuaResult<()> {
        let path = format!("{}/main.lua", self.scripts_dir);
        let code = fs::read_to_string(path)?;
        self.lua.load(&code).exec()
    }

    /* Calls start() in main.Lua 
     * Requires users to call start() for other files if they have more
     * Might switch later but would require preloading all modules which might be a pain
     */
    pub fn call_start(&self) -> LuaResult<()> {
        let globals = self.lua.globals();
        let start: LuaFunction = globals.get("start")?;
        start.call(())
    }

    pub fn call_update(&self, dt: u128) -> LuaResult<()> {
        let globals = self.lua.globals();
        let update: LuaFunction = globals.get("update")?;
        update.call(dt)
    }
}
