use mlua::prelude::LuaResult;
use rico_32::engine::rico::RicoEngine;

fn main() -> LuaResult<()> {
    let engine = RicoEngine::new()?;
    let _ = engine.start();
    Ok(())
}
