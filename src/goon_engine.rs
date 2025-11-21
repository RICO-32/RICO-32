use mlua::prelude::*;

use crate::game_engine::{GameEngine};

const SCREEN_SIZE: usize = 128;

pub struct GoonEngine{
    pixels: [[i32; SCREEN_SIZE]; SCREEN_SIZE],
    game_engine: GameEngine,
    state: i32
}

impl GoonEngine{
    pub fn new() -> LuaResult<Self>{
        Ok(GoonEngine{
            pixels: [[2; SCREEN_SIZE]; SCREEN_SIZE],
            game_engine: GameEngine::new()?,
            state: 0
       })
    }

    pub fn update(&mut self) -> LuaResult<()> {
        match self.state {
            0 => Ok(self.game_engine.update()?),
            _ => Ok(())
        }
    }
}
