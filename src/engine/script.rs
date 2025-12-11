use mlua::prelude::*;

use mlua::StdLib;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::scripting::lua::{LuaAPI, LuaAPIHandle};

pub struct ScriptEngine {
    pub lua: Lua,
    scripts: HashMap<String, String>,
}

impl ScriptEngine {
    pub fn new(scripts: HashMap<String, String>) -> Self {
        let options = LuaOptions::new();
        let lua = Lua::new_with(StdLib::ALL_SAFE, options).expect("Could not load lua state");

        let engine = ScriptEngine { lua, scripts };

        engine.register_loader().expect("Could not register Lua module loader");

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
        let lua = &self.lua;
        let scripts = self.scripts.clone();

        let loader = lua.create_function(move |lua, module: String| {
            let path1 = format!("{}.lua", module.replace(".", "/"));
            let path2 = format!("{}/init.lua", module.replace(".", "/"));
            //
            // Try both module.lua and module/init.lua
            let code = scripts.get(&path1).or_else(|| scripts.get(&path2));

            match code {
                Some(src) => {
                    let func = lua.load(src.as_str()).into_function()?;

                    Ok(mlua::MultiValue::from_vec(vec![
                        mlua::Value::Function(func),
                        mlua::Value::String(lua.create_string(&path1)?),
                    ]))
                }
                None => Ok(mlua::MultiValue::from_vec(vec![
                    mlua::Value::Nil,
                    mlua::Value::String(
                        lua.create_string(format!("module '{}' not found", module))?,
                    ),
                ])),
            }
        })?;

        let globals = lua.globals();
        let package: LuaTable = globals.get("package")?;
        let searchers: LuaTable = package.get("searchers")?;
        searchers.raw_insert(1, loader)?;

        Ok(())
    }

    //Execs the main lua file, mainly for global stuff
    pub fn boot(&self) -> LuaResult<()> {
        let path = "main.lua";
        match self.scripts.get(path) {
            Some(code) => self.lua.load(code).exec(),
            None => Err(mlua::Error::RuntimeError("Could not find main file".to_string())),
        }
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
