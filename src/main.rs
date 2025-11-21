use mlua::prelude::*;

mod script_engine;
mod game_engine;
mod goon_engine;
use goon_engine::GoonEngine;

fn main() -> LuaResult<()> {
    let mut engine = GoonEngine::new()?;

    loop {
        engine.update()?;
    }
}
