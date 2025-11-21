use mlua::prelude::*;
use mlua::StdLib;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, thread, time};
use std::time::Instant;

use crate::sprite::{Sprite, SpriteHandle};

const MILLIS_IN_SEC: u64 = 1000;
const BASE_FPS: i32 = 60;

pub struct ScriptEngine {
    lua: Lua,
    scripts_dir: String,
    last_time: Instant,
    frame_rate: Rc<RefCell<i32>>,
    sprites: Rc<RefCell<Vec<Rc<RefCell<Sprite>>>>>,
}

impl ScriptEngine {
    /* Will also handle rebooting
     * Redo the engine initialization whenever restarting the game in the engine
     * Call new, boot, and call_start
     */
    pub fn new(scripts_dir: &str) -> LuaResult<Self> {
        let options = LuaOptions::new();
        let lua = Lua::new_with(StdLib::ALL_SAFE, options).expect("Could not load lua state");

        let mut engine = ScriptEngine {
            lua,
            scripts_dir: String::from(scripts_dir),
            last_time: Instant::now(),
            frame_rate: Rc::from(RefCell::from(BASE_FPS)),
            sprites: Rc::from(RefCell::from(Vec::new())),
        };

        engine.register_api()?;
        engine.register_loader()?;

        Ok(engine)
    }

    /* Expose the sprites API to Lua
     * Functions defined in readme
     */
    fn register_sprites(&mut self) -> LuaResult<()>{
        let globals = self.lua.globals();

        let sprites_vec = self.sprites.clone();
        let sprite_table = self.lua.create_table()?;
        let new_fn = self.lua.create_function(move |_, (file, x, y, size): (String, i32, i32, i32)| {
                //LOTS of bullshit to use references instead of direct objects
                //Necessary cause we want both Rust and Lua to have full access to these objects
                let sprite = Rc::new(RefCell::new(Sprite::new(file, x, y, size)));
                sprites_vec.borrow_mut().push(Rc::clone(&sprite));
                Ok(SpriteHandle(sprite)) 
            }
        )?;
        sprite_table.set("new", new_fn)?;
        globals.set("Sprite", sprite_table)?;
        Ok(())
    }

    //Define all lua API functions here 
    fn register_api(&mut self) -> LuaResult<()> {
        let globals = self.lua.globals();

        let _ = self.register_sprites();
        
        globals.set(
            "log",
            self.lua.create_function(|_, msg: String| {
                println!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        //Mutex bs to deal with lua functions being global, avoids self going out of scope
        let frame_rate = self.frame_rate.clone();
        globals.set(
            "set_frame_rate",
            self.lua.create_function(move |_, msg: String| {
                let x = msg.parse::<i32>()
                    .map_err(|_| mlua::Error::RuntimeError(format!("Invalid frame rate: {}", msg)))?;
                *frame_rate.borrow_mut() = x;
                Ok(())
            })?,
        )?;



        Ok(())
    }

    /* Only way I could think to allow modularization
     * Just a wrapper to load in lua scripts and insert into main
     * Keeps Lua API the same 
     */
    fn register_loader(&self) -> LuaResult<()> {
        let scripts = self.scripts_dir.clone();
        let lua = &self.lua;

        let loader = lua.create_function(move |lua, module: String| {
            let path1 = format!("{}/{}.lua", scripts, module.replace(".", "/"));
            let path2 = format!("{}/{}/init.lua", scripts, module.replace(".", "/"));

            let code = fs::read_to_string(&path1)
                .or_else(|_| fs::read_to_string(&path2))
                .or_else(|_| Ok::<String, String>(String::from("")))
                .unwrap();

            let func = lua.load(&code).into_function()?;
            Ok(func)
        })?;

        //Simply overwrite the inbuilt package loader with ours
        let globals = self.lua.globals();
        let package: LuaTable = globals.get("package")?;
        let searchers: LuaTable = package.get("searchers")?;
        searchers.raw_insert(1, loader)?;

        Ok(())
    }

    //Execs the main lua file, mainly for global stuff
    pub fn boot(&self) -> LuaResult<()> {
        let path = format!("{}/main.lua", self.scripts_dir);
        let code = fs::read_to_string(path)?;
        self.lua.load(&code).exec()
    }

    /* Calls start() in main.Lua 
     * Requires users to call start() for other files if they have more
     * Might switch later but would require preloading all modules which might be a pain
     */
    pub fn call_start(&self) -> LuaResult<()> {
        let globals = self.lua.globals();
        let start: LuaFunction = globals.get("start")?;
        start.call(())
    }

    //Calls update with the delta time and syncs frame rate
    pub fn update(&mut self) -> LuaResult<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_millis();
        self.last_time = now;
        self.sync();
        for sprite in self.sprites.borrow().iter(){
            sprite.borrow().draw();
        }
        self.call_update(dt)
    }

    //Artificially syncs frame rate, idk a better way to do this
    fn sync(&self){
        let sync_wait = MILLIS_IN_SEC/(*self.frame_rate.borrow()) as u64;
        thread::sleep(time::Duration::from_millis(sync_wait));
    }

    fn call_update(&self, dt: u128) -> LuaResult<()> {
        let globals = self.lua.globals();
        let update: LuaFunction = globals.get("update")?;
        update.call(dt)
    }
}
