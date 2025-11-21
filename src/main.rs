use mlua::prelude::*;

mod script_engine;

mod game_engine;
use game_engine::GameEngine;

fn main() -> LuaResult<()> {
    let engine = GameEngine::new()?;

    loop {
        engine.borrow_mut().update()?;
    }
}
