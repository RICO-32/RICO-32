use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use crate::engine::console::ConsoleEngine;
use crate::engine::script::ScriptEngine;
use crate::scripting::lua::{LogTypes, LuaAPI};
use crate::time::sync;

pub const BASE_FPS: i32 = 60;

//This class is literally just an orchestrator between the actual lua game and console
pub struct GameEngine {
    pub script_engine: ScriptEngine,
    pub console_engine: ConsoleEngine,
    pub lua_api: Rc<RefCell<LuaAPI>>,
}

impl Default for GameEngine {
    fn default() -> Self {
        let script_engine = ScriptEngine::new("scripts");

        let mut eng = GameEngine {
            script_engine,
            console_engine: ConsoleEngine::default(),
            lua_api: Rc::from(RefCell::from(LuaAPI::default())),
        };

        //Register all loaders if something errors just print to console screen
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
}

impl GameEngine {
    fn add_errors<T: Error>(&mut self, err: T) {
        let msg = err.to_string();

        //Filter .rs stuff so the user only sees their part of the code that messed up
        let mut cleaned = msg
            .lines()
            .filter(|line| !line.contains(".rs"))
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        //Just cleaner imo
        cleaned.push(" ".to_string());
        for chunk in cleaned.iter() {
            self.lua_api.borrow_mut().add_log(LogTypes::Err(chunk.to_string()));
        }
    }

    pub fn update(&mut self) {
        //Halting is in console so make sure thats not true
        if !self.console_engine.halted {
            let dt = sync(&mut self.console_engine.last_time, self.lua_api.borrow().frame_rate);

            if let Err(err) = self.script_engine.call_update(dt) {
                self.add_errors(err);
            }
        }

        //Might wanna store logs in the actual console at some point but thats kinda janky
        self.console_engine.update(&self.lua_api.borrow().logs);

        //Nuclear option but I gen dont see the problem we do wanna restart everything
        if self.console_engine.restart {
            *self = GameEngine::default();
        }
    }
}
