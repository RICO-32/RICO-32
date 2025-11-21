use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use mlua::prelude::LuaResult;

use crate::script_engine::ScriptEngine;

const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u64 = 1000;

/* Enum of all command types
 * Add command here for any new defined commands
 */
pub enum Commands{
    Log(String),
    SetFrameRate(i32),
    Draw(i32, i32, String),
    Button(i32, i32, String),
    PrintScr(i32, i32, String),
}

pub struct GameEngine{
    script_engine: ScriptEngine,
    last_time: Instant,
    frame_rate: Rc<RefCell<i32>>,
    commands: Rc<RefCell<Vec<Commands>>>
}

impl GameEngine{
    pub fn new() -> LuaResult<Self> {
        let frame_rate = Rc::from(RefCell::from(BASE_FPS));
        let engine = ScriptEngine::new("scripts")?;

        //Make engine using script
        let mut game_engine = GameEngine {
            script_engine: engine,
            last_time: Instant::now(),
            frame_rate: frame_rate.clone(),
            commands: Rc::from(RefCell::from(Vec::new())),
        };

        //All init functions
        game_engine.script_engine.boot()?;
        game_engine.script_engine.register_api(game_engine.commands.clone())?;
        game_engine.script_engine.call_start()?;

        Ok(game_engine)
    }

    //Place holder functions
    pub fn draw(&mut self, x: i32, y: i32, file: String){
        println!("Drawing {} at {} {}", file, x, y);
    }

    pub fn button(&mut self, x: i32, y: i32, msg: String){
        println!("Making button {} at {} {}", msg, x, y);
    }

    pub fn print_src(&mut self, x: i32, y: i32, msg: String){
        println!("Adding text {} at {} {}", msg, x, y);
    }

    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    pub fn update(&mut self) -> LuaResult<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_millis();
        self.last_time = now;
        self.sync();
        self.run_commands();
        self.script_engine.call_update(dt)
    }

    //Run all commands and free up vector
    fn run_commands(&mut self){
        for command in self.commands.clone().borrow().iter(){
            match command{
                Commands::Log(msg) => println!("{}", format!("[Lua] {}", msg)),
                Commands::SetFrameRate(rate) => *self.frame_rate.borrow_mut() = *rate,
                Commands::Draw(x, y, file) => self.draw(*x, *y, file.clone()),
                Commands::PrintScr(x, y, msg) => self.print_src(*x, *y, msg.clone()),
                Commands::Button(x, y, msg) => self.button(*x, *y, msg.clone()),
            }
        }
        self.commands.borrow_mut().clear();
    }
    
    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&self){
        let sync_wait = MILLIS_IN_SEC/(*self.frame_rate.borrow()) as u64;
        thread::sleep(time::Duration::from_millis(sync_wait));
    }

}
