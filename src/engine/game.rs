use std::cell::Ref;
use std::{cell::RefCell, rc::Rc};

use mlua::prelude::LuaResult;

use crate::engine::console::ConsoleEngine;
use crate::scripting::lua::LuaAPI;
use crate::engine::script::ScriptEngine;
use crate::engine::rico::{PixelsType, ScreenEngine};
use crate::time::sync;

pub const BASE_FPS: i32 = 60;

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


    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    pub fn update(&mut self) {
        let dt = sync(&mut self.console_engine.last_time, self.lua_api.borrow().frame_rate);
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
