use mlua::prelude::*;
use mlua::StdLib;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs};

use crate::game_engine::Commands;

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

    //Define all lua API functions here 
    pub fn register_api(&mut self, commands: Rc<RefCell<Vec<Commands>>>) -> LuaResult<()> {
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
            self.lua.create_function(move |_, msg: String| {
                let x = msg.parse::<i32>()
                    .map_err(|_| mlua::Error::RuntimeError(format!("Invalid frame rate: {}", msg)))?;
                com_rc.borrow_mut().push(Commands::SetFrameRate(x));
                Ok(())
            })?,
        )?;

        let com_rc = commands.clone();
        let _ = self.populate_lua_api("draw", move |x, y, msg| {
            com_rc.borrow_mut().push(Commands::Draw(x, y, msg));
            Ok(())
        });

        let com_rc = commands.clone();
        let _ = self.populate_lua_api("print_scr", move |x, y, msg| {
            com_rc.borrow_mut().push(Commands::PrintScr(x, y, msg));
            Ok(())
        });

        let com_rc = commands.clone();
        let _ = self.populate_lua_api("button", move |x, y, msg| {
            com_rc.borrow_mut().push(Commands::Button(x, y, msg));
            Ok(())
        });

        Ok(())
    }

    fn populate_lua_api<F>(&self, name: &str, f: F) -> LuaResult<()>
    where F: 'static + Fn(i32, i32, String) -> LuaResult<()>{
        let globals = self.lua.globals();
        globals.set(
            name,
            self.lua.create_function(move |_, (x, y, msg): (i32, i32, String)| {
                f(x, y, msg)
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
