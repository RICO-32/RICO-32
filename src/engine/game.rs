use std::cell::Ref;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use crate::engine::console::ConsoleEngine;
use crate::engine::rico::{PixelsType, ScreenEngine};
use crate::engine::script::ScriptEngine;
use crate::scripting::lua::{LogTypes, LuaAPI};
use crate::time::sync;

pub const BASE_FPS: i32 = 60;

pub struct GameEngine {
    pub script_engine: ScriptEngine,
    pub console_engine: ConsoleEngine,
    pub lua_api: Rc<RefCell<LuaAPI>>,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GameEngine {
    pub fn new() -> Self {
        let script_engine = ScriptEngine::new("scripts");

        let mut eng = GameEngine {
            script_engine,
            console_engine: ConsoleEngine::new(),
            lua_api: Rc::from(RefCell::from(LuaAPI::default())),
        };

        if let Err(err) = eng.script_engine.register_api(eng.lua_api.clone()) {
            eng.add_errors(err);
        };
        if let Err(err) = eng.script_engine.boot() {
            eng.add_errors(err);
        };
        if let Err(err) = eng.script_engine.call_start() {
            eng.add_errors(err);
        };

        eng
    }

    fn add_errors<T: Error>(&mut self, err: T) {
        let msg = err.to_string();

        let mut cleaned = msg
            .lines()
            .filter(|line| !line.contains(".rs"))
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        cleaned.push(" ".to_string());
        for chunk in cleaned.iter() {
            self.lua_api.borrow_mut().add_log(LogTypes::Err(chunk.to_string()));
        }
    }

    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    pub fn update(&mut self) {
        if !self.console_engine.halted {
            let dt = sync(&mut self.console_engine.last_time, self.lua_api.borrow().frame_rate);

            if let Err(err) = self.script_engine.call_update(dt) {
                self.add_errors(err);
            }

            if self.lua_api.borrow().mouse.just_pressed {
                self.lua_api.borrow_mut().mouse.just_pressed = false;
            };
            self.lua_api.borrow_mut().keyboard.keys_just_pressed.clear();
        }

        self.console_engine.update(&self.lua_api.borrow().logs);

        if self.console_engine.restart {
            *self = GameEngine::new();
        }
    }
}

impl ScreenEngine for GameEngine {
    type Pixels<'a> = Ref<'a, PixelsType>;
    fn pixels(&self) -> Self::Pixels<'_> {
        Ref::map(self.lua_api.borrow(), |api| &api.pixels)
    }
}
