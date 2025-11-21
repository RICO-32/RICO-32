use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use mlua::prelude::LuaResult;

use crate::script_engine::ScriptEngine;

const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u64 = 1000;

pub struct GameEngine{
    script_engine: ScriptEngine,
    last_time: Instant,
    frame_rate: Rc<RefCell<i32>>,
}

impl GameEngine{
    pub fn new() -> LuaResult<Self> {
        let frame_rate = Rc::from(RefCell::from(BASE_FPS));
        let engine = ScriptEngine::new("scripts", frame_rate.clone())?;

        engine.boot()?;
        engine.call_start()?;

        Ok(GameEngine{
            script_engine: engine,
            last_time: Instant::now(),
            frame_rate: frame_rate
        })
    }

    pub fn update(&mut self) -> LuaResult<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_millis();
        self.last_time = now;
        self.sync();
        self.script_engine.call_update(dt)
    }
    
    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&self){
        let sync_wait = MILLIS_IN_SEC/(*self.frame_rate.borrow()) as u64;
        thread::sleep(time::Duration::from_millis(sync_wait));
    }

}
