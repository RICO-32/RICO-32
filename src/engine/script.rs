use mlua::prelude::*;

use mlua::StdLib;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use crate::scripting::lua::{LuaAPI, LuaAPIHandle};

pub struct ScriptEngine {
    pub lua: Lua,
    scripts_dir: String,
}

impl ScriptEngine {
    pub fn new(scripts_dir: &str) -> Self {
        let options = LuaOptions::new();
        let lua = Lua::new_with(StdLib::ALL_SAFE, options).expect("Could not load lua state");

        let engine = ScriptEngine { lua, scripts_dir: String::from(scripts_dir) };

        engine.register_loader().expect("Could not register Lua moldule loader");

        engine
    }

    //Maybe we should move this to the actual lua api but wtv for now
    pub fn register_api(&mut self, lua_api: Rc<RefCell<LuaAPI>>) -> LuaResult<()> {
        let lua_state = self.lua.create_userdata(LuaAPIHandle(lua_api.clone()))?;
        self.lua.globals().set("rico", lua_state)?;
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
