use mlua::prelude::*;
use mlua::{StdLib};
use std::{fs, thread, time};
use std::time::Instant;

pub struct ScriptEngine {
    lua: Lua,
    scripts_dir: String,
    time: Instant,
    frame_rate: i32
}

impl ScriptEngine {
    pub fn new(scripts_dir: impl Into<String>) -> LuaResult<Self> {
        let options = LuaOptions::new();
        let lua = Lua::new_with(StdLib::ALL_SAFE, options).expect("Could not load lua state");

        let engine = ScriptEngine {
            lua,
            scripts_dir: scripts_dir.into(),
            time: Instant::now(),
            frame_rate: 60
        };

        engine.register_api()?;
        engine.register_loader()?;

        Ok(engine)
    }

    fn register_api(&self) -> LuaResult<()> {
        let globals = self.lua.globals();

        globals.set(
            "log",
            self.lua.create_function(|_, msg: String| {
                println!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        Ok(())
    }

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

        let globals = self.lua.globals();
        let package: LuaTable = globals.get("package")?;
        let searchers: LuaTable = package.get("searchers")?;
        searchers.raw_insert(1, loader)?;

        Ok(())
    }

    pub fn boot(&self) -> LuaResult<()> {
        let path = format!("{}/main.lua", self.scripts_dir);
        let code = fs::read_to_string(path)?;
        self.lua.load(&code).exec()
    }

    pub fn call_start(&self) -> LuaResult<()> {
        let globals = self.lua.globals();
        let start: LuaFunction = globals.get("start")?;
        start.call::<()>(())
    }

    pub fn update(&mut self) -> LuaResult<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.time).as_millis();
        self.time = now;
        thread::sleep(time::Duration::from_millis(1000/self.frame_rate as u64));
        self.call_update(dt)
    }

    fn call_update(&self, dt: u128) -> LuaResult<()> {
        let globals = self.lua.globals();
        let update: LuaFunction = globals.get("update")?;
        update.call::<()>(dt)
    }
}


fn main() -> LuaResult<()> {
    let mut engine = ScriptEngine::new("scripts")?;

    engine.boot()?;
    engine.call_start()?;

    loop {
        engine.update()?;
    }
}
