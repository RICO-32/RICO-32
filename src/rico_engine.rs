use std::{cell::{Ref, RefCell}, rc::Rc, usize};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{game_engine::GameEngine, utils::{keyboard::Keyboard, mouse::MousePress}};
use crate::utils::colors::COLORS;

pub const SCREEN_SIZE: usize = 128;
pub const SCALE: usize = 4;
pub const WINDOW_WIDTH: usize = SCREEN_SIZE * SCALE;
pub const WINDOW_HEIGHT: usize = SCREEN_SIZE * 2 * SCALE;

pub type PixelsType = Vec<Vec<COLORS>>;

/* All screen engines must implement
 * Game for now, sprite in the future, maybe IDE
 */
pub trait ScreenEngine {
    fn pixels(&self) -> Ref<PixelsType>;
}

enum StateEngines {
    GameEngine
}

/* Add bindings for diff engines in this struct
 * All engines are different screens on the console
 * Engines should implement the Engine trait
 */
pub struct RicoEngine{
    game_engine: GameEngine,
    state_engine: StateEngines,
}

impl RicoEngine{
    pub fn new() -> LuaResult<Self>{
        let engine = RicoEngine{
            game_engine: GameEngine::new()?,
            state_engine: StateEngines::GameEngine,
        };

        Ok(engine)
    }

    //Base boot function, needs to take in whole self cause borrowing bs
    pub fn start(self) -> Result<(), Box<dyn std::error::Error>>{
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("RICO-32")
            .with_resizable(false)
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64))
            .build(&event_loop)?;

        let surface_texture = SurfaceTexture::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, &window);
        let mut pixels = Pixels::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, surface_texture)?;

        let engine_rc = Rc::new(RefCell::new(self)).clone();
        // Event loop: Poll so we run as fast as possible and continuously request redraws
        event_loop.run(move |event, _, control_flow| {
            // Poll loop -> render as fast as possible
            *control_flow = ControlFlow::Poll;

            let mut engine = engine_rc.borrow_mut();
            match event {
                Event::RedrawRequested(_) => {
                    //Pass in buffer and redraw all based pixels every frame
                    let buffer = pixels.frame_mut();
                    let _ = engine.update(buffer);

                    if let Err(_) = pixels.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                //Redraw every frame
                Event::MainEventsCleared => {
                    window.request_redraw();
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match engine.state_engine{
                                StateEngines::GameEngine => {
                                    let mut lua_api = engine.game_engine.lua_api.borrow_mut();
                                    bind_keyboard(&mut lua_api.keyboard, input.state, keycode);
                                }
                            }

                            // exit on ESC
                            if keycode == winit::event::VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    },

                    WindowEvent::MouseInput { button, state, .. } => {
                        match engine.state_engine{
                            StateEngines::GameEngine => {
                                bind_mouse_input(&mut engine.game_engine.lua_api.borrow_mut().mouse, button, state);
                                bind_mouse_input(&mut engine.game_engine.log_engine.mouse, button, state);
                            }
                        };
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        match engine.state_engine {
                            StateEngines::GameEngine => {
                                let scale = window.scale_factor();
                                let logical = position.to_logical::<f32>(scale);

                                bind_mouse_move(&mut engine.game_engine.lua_api.borrow_mut().mouse, logical, 0, 0, WINDOW_WIDTH, WINDOW_WIDTH);
                                bind_mouse_move(&mut engine.game_engine.log_engine.mouse, logical, 0, WINDOW_WIDTH, WINDOW_WIDTH, WINDOW_WIDTH);
                            }
                        }
                    }

                    _ => {}
                },
                _ => {}
            }
        });
    }

    //Make sure to update engines here based on which screen it's on
    pub fn update(&mut self, buffer: &mut [u8]) -> LuaResult<()> {
        return match self.state_engine {
            StateEngines::GameEngine => {
                if !self.game_engine.log_engine.halted {
                    self.game_engine.update();
                    let pixels = self.game_engine.pixels();

                    copy_pixels_into_buffer(pixels, buffer, 0, 0);
                }
                
                self.game_engine.log_engine.update(&self.game_engine.lua_api.borrow().logs);

                let pixels = self.game_engine.log_engine.pixels();
                copy_pixels_into_buffer(pixels, buffer, 0, WINDOW_WIDTH);
                
                if self.game_engine.log_engine.restart {
                    self.game_engine = GameEngine::new()?;
                }

                Ok(())
            }
        }
    }
}

//Hydrate the screen based on scaling factors and stuff
fn copy_pixels_into_buffer(pixels: Ref<PixelsType>, buffer: &mut [u8], start_x: usize, start_y: usize){
    let height = pixels.len();
    let width = pixels[0].len();
    for y in 0..height{
        for x in 0..width{
            for dy in 0..SCALE{
                for dx in 0..SCALE{
                    let idx = ((y * SCALE + dy) * WINDOW_WIDTH as usize + (x * SCALE + dx)) + start_y * WINDOW_WIDTH + start_x;
                    let COLORS(r, g, b, a) = pixels[y][x];
                    buffer[idx*4..idx*4+4].copy_from_slice(&[r, g, b, a]);
                }
            }
        }
    }
}

fn bind_keyboard(keyboard: &mut Keyboard, state: ElementState, keycode: VirtualKeyCode){
    match state {
        ElementState::Pressed => {
            if !keyboard.keys_pressed.contains(&keycode) {
                keyboard.keys_just_pressed.insert(keycode);
            }
            keyboard.keys_pressed.insert(keycode);
        }
        ElementState::Released => {
            keyboard.keys_pressed.remove(&keycode);
        }
    }
}

fn bind_mouse_input(mouse: &mut MousePress, button: MouseButton, state: ElementState){
    if button == MouseButton::Left {
        match state {
            ElementState::Pressed => {
                mouse.pressed = true;
                mouse.just_pressed = true;
            },
            ElementState::Released => {
                mouse.pressed = false;
                mouse.just_pressed = false;
            },
        }
    }
}

fn check_mouse_bounds(mouse: &mut MousePress, start_x: usize, start_y: usize, width: usize, height: usize) -> bool {
    let cur_x = mouse.x as usize;
    let cur_y = mouse.y as usize;
    if cur_x < start_x || cur_x > start_x + width || cur_y < start_y || cur_y > start_y + height {
        mouse.pressed = false;
        mouse.just_pressed = false;
        mouse.x = -1;
        mouse.y = -1;
        return true;
    }

    false
}

fn bind_mouse_move(mouse: &mut MousePress, logical_position: LogicalPosition<f32>, start_x: usize, start_y: usize, width: usize, height: usize){
    mouse.x = logical_position.x as i32;
    mouse.y = logical_position.y as i32;

    if !check_mouse_bounds(mouse, start_x, start_y, width, height){
        mouse.x -= start_x as i32;
        mouse.y -= start_y as i32;

        mouse.x /= SCALE as i32;
        mouse.y /= SCALE as i32;
    };
}
