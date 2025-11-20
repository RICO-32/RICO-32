use mlua::prelude::*;

mod script_engine;
use script_engine::ScriptEngine;

fn main() -> LuaResult<()> {
    let mut engine = ScriptEngine::new("scripts")?;

    engine.boot()?;
    engine.call_start()?;

    loop {
        engine.update()?;
    }
}
