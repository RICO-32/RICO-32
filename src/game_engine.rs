use std::cell::Ref;
use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use mlua::prelude::LuaResult;

use crate::console_engine::ConsoleEngine;
use crate::lua_api::LuaAPI;
use crate::script_engine::ScriptEngine;
use crate::rico_engine::{PixelsType, ScreenEngine};

pub const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u128 = 1000;

pub struct GameEngine{
    pub script_engine: ScriptEngine,
    pub console_engine: ConsoleEngine,
    pub lua_api: Rc<RefCell<LuaAPI>>
}

impl GameEngine{
    pub fn new() -> LuaResult<Self> {
        let script_engine = ScriptEngine::new("scripts")?;

        let mut eng = GameEngine {
            script_engine,
            console_engine: ConsoleEngine::new(),
            lua_api: Rc::from(RefCell::from(LuaAPI::default()))
        };

        eng.script_engine.boot()?;
        eng.script_engine.register_api(eng.lua_api.clone())?;
        eng.script_engine.call_start()?;

       Ok(eng)
    }


    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&mut self) -> u128 {
        let frame_rate = self.lua_api.borrow().frame_rate;
        if frame_rate <= 0 {
            let now = Instant::now();
            let dt = self.console_engine.last_time.elapsed().as_millis();
            self.console_engine.last_time = now;
            return dt;
        }

        let target_frame_time = time::Duration::from_millis((MILLIS_IN_SEC as f64 / frame_rate as f64) as u64);
        let elapsed_time = self.console_engine.last_time.elapsed();

        if elapsed_time < target_frame_time {
            thread::sleep(target_frame_time - elapsed_time);
        }

        let dt = self.console_engine.last_time.elapsed().as_millis();
        self.console_engine.last_time = Instant::now();
        dt
    }
    
    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    pub fn update(&mut self) {
        let dt = self.sync();
        let _ = self.script_engine.call_update(dt);
        if self.lua_api.borrow().mouse.just_pressed {
            self.lua_api.borrow_mut().mouse.just_pressed = false;
        };
        self.lua_api.borrow_mut().keyboard.keys_just_pressed.clear();
    }
}

impl ScreenEngine for GameEngine{
    type Pixels<'a> = Ref<'a, PixelsType>;
    fn pixels(&self) -> Self::Pixels<'_>{
        Ref::map(self.lua_api.borrow(), |api| &api.pixels)
    }
}
