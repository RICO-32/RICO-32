use mlua::prelude::LuaResult;
mod script_engine;
mod game_engine;
mod goon_engine;
mod utils;
use goon_engine::GoonEngine;

fn main() -> LuaResult<()> {
    let engine = GoonEngine::new()?;
    let _ = engine.start();
    Ok(())
}
