use mlua::prelude::LuaResult;
mod script_engine;
mod game_engine;
mod rico_engine;
mod utils;
use rico_engine::RicoEngine;

fn main() -> LuaResult<()> {
    let engine = RicoEngine::new()?;
    let _ = engine.start();
    Ok(())
}
