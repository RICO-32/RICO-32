use std::{cell::RefCell, rc::Rc, time::Instant};
use std::{thread, time};

use mlua::prelude::LuaResult;

use crate::bitmap::BITMAP;
use crate::colors::COLORS;
use crate::script_engine::ScriptEngine;
use crate::goon_engine::{PixelsType, ScreenEngine};

const BASE_FPS: i32 = 60;
const MILLIS_IN_SEC: u128 = 1000;

/* Enum of all command types
 * Add command here for any new defined commands
 */
pub enum Commands{
    Log(String),
    SetFrameRate(i32),
    Draw(usize, usize, String),
    Button(usize, usize, String),
    PrintScr(usize, usize, COLORS, String),
}

pub struct GameEngine{
    script_engine: ScriptEngine,
    last_time: Instant,
    frame_rate: Rc<RefCell<i32>>,
    commands: Rc<RefCell<Vec<Commands>>>,
    pixels: Rc<RefCell<PixelsType>>,
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
            pixels: Rc::from(RefCell::from(COLORS::pixels())),
        };

        //All init functions
        game_engine.script_engine.boot()?;

        game_engine.script_engine.register_api_commands(game_engine.commands.clone())?;
        game_engine.script_engine.register_api_in_place(game_engine.pixels.clone())?;
        game_engine.script_engine.call_start()?;

        Ok(game_engine)
    }

    //Place holder functions
    pub fn draw(&mut self, x: usize, y: usize, file: String){
        println!("Drawing {} at {} {}", file, x, y);
    }

    pub fn button(&mut self, x: usize, y: usize, msg: String){
        println!("Making button {} at {} {}", msg, x, y);
    }

    pub fn print_scr(&mut self, x: usize, y: usize, col: COLORS, msg: String){
        let pixels_rc = self.pixels.clone();
        for i in 0..msg.len(){
            let c = msg.as_bytes().iter().nth(i).unwrap();
            let mut idx: usize = (*c).into();
            idx -= 32;
            if idx >= BITMAP.len() {
                idx = 0;
            }

            for dx in 0..8{
                for dy in 0..8{
                    if BITMAP[idx][dy] >> (7-dx) & 1 == 1{
                        pixels_rc.borrow_mut()[y+dy][x+dx+i*8] = col;
                    }
                }
            }
        }
    }

    //Run all commands and free up vector
    fn run_commands(&mut self){
        for command in self.commands.clone().borrow().iter(){
            match command{
                Commands::Log(msg) => println!("{}", format!("[Lua] {}", msg)),
                Commands::SetFrameRate(rate) => *self.frame_rate.borrow_mut() = *rate,
                Commands::Draw(x, y, file) => self.draw(*x, *y, file.clone()),
                Commands::PrintScr(x, y, col, msg) => self.print_scr(*x, *y, *col, msg.clone()),
                Commands::Button(x, y, msg) => self.button(*x, *y, msg.clone()),
            }
        }
        self.commands.borrow_mut().clear();
    }

    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&mut self) -> u128 {
        let frame_rate = *self.frame_rate.borrow();
        if frame_rate <= 0 {
            let now = Instant::now();
            let dt = self.last_time.elapsed().as_millis();
            self.last_time = now;
            return dt;
        }

        let target_frame_time = time::Duration::from_millis((MILLIS_IN_SEC as f64 / frame_rate as f64) as u64);
        let elapsed_time = self.last_time.elapsed();

        if elapsed_time < target_frame_time {
            thread::sleep(target_frame_time - elapsed_time);
        }

        let dt = self.last_time.elapsed().as_millis();
        self.last_time = Instant::now();
        dt
    }
}

impl ScreenEngine for GameEngine{
    fn pixels(&self) -> Rc<RefCell<PixelsType>>{
        self.pixels.clone()
    }
    //Syncs with frame rate, runs all queued up commands from this prev frame, calls main update
    fn update(&mut self) -> LuaResult<()> {
        let dt = self.sync();
        self.run_commands();
        self.script_engine.call_update(dt)
    }
}
