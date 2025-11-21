use mlua::prelude::*;

mod script_engine;

mod game_engine;
use game_engine::GameEngine;

fn main() -> LuaResult<()> {
    let mut engine = GameEngine::new()?;

    loop {
        engine.update()?;
    }
}
