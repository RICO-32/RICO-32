use std::{cell::RefCell, rc::Rc, usize};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{game_engine::GameEngine};
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
    fn pixels(&self) -> Rc<RefCell<PixelsType>>;
    fn update(&mut self);    
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

            match event {
                Event::RedrawRequested(_) => {
                    //Pass in buffer and redraw all based pixels every frame
                    let buffer = pixels.get_frame_mut();
                    let _ = engine_rc.borrow_mut().update(buffer);

                    if let Err(_) = pixels.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                //Redraw every frame
                Event::MainEventsCleared => {
                    window.request_redraw();
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match engine_rc.borrow().state_engine{
                                StateEngines::GameEngine => {
                                    match input.state {
                                        winit::event::ElementState::Pressed => {
                                            engine_rc.borrow().game_engine.keys_pressed.borrow_mut().insert(keycode);
                                        }
                                        winit::event::ElementState::Released => {
                                            engine_rc.borrow().game_engine.keys_pressed.borrow_mut().remove(&keycode);
                                        }
                                    }
                                }
                            }

                            // exit on ESC
                            if keycode == winit::event::VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    },

                    WindowEvent::MouseInput { button, state, .. } => {
                        match engine_rc.borrow().state_engine{
                            StateEngines::GameEngine => {
                                match state {
                                    winit::event::ElementState::Pressed => {
                                        if button == MouseButton::Left {
                                            engine_rc.borrow().game_engine.mouse.borrow_mut().pressed = true;
                                            engine_rc.borrow().game_engine.mouse.borrow_mut().just_pressed = true;

                                            engine_rc.borrow().game_engine.log_engine.mouse.borrow_mut().pressed = true;
                                            engine_rc.borrow().game_engine.log_engine.mouse.borrow_mut().just_pressed = true;
                                        }
                                    }
                                    winit::event::ElementState::Released => {
                                        if button == MouseButton::Left {
                                            engine_rc.borrow().game_engine.mouse.borrow_mut().pressed = false;
                                            engine_rc.borrow().game_engine.log_engine.mouse.borrow_mut().pressed = false;
                                        }
                                    }
                                }
                            }
                        };
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        match engine_rc.borrow().state_engine {
                            StateEngines::GameEngine => {
                                let scale = window.scale_factor();
                                let logical = position.to_logical::<f32>(scale);
                                let mouse_rc = engine_rc.borrow().game_engine.mouse.clone();
                                mouse_rc.borrow_mut().x = logical.x as i32 / SCALE as i32;
                                mouse_rc.borrow_mut().y = logical.y as i32 / SCALE as i32;

                                if mouse_rc.borrow().x > SCREEN_SIZE as i32 || mouse_rc.borrow().y > SCREEN_SIZE as i32 {
                                    mouse_rc.borrow_mut().x = -1;
                                    mouse_rc.borrow_mut().y = -1;
                                }

                                let mouse_rc = engine_rc.borrow().game_engine.log_engine.mouse.clone();
                                mouse_rc.borrow_mut().x = logical.x as i32 / SCALE as i32;
                                mouse_rc.borrow_mut().y = logical.y as i32 / SCALE as i32;

                                if mouse_rc.borrow().x > SCREEN_SIZE as i32 || mouse_rc.borrow().y <= SCREEN_SIZE as i32 {
                                    mouse_rc.borrow_mut().x = -1;
                                    mouse_rc.borrow_mut().y = -1;
                                } else {
                                    mouse_rc.borrow_mut().y -= SCREEN_SIZE as i32;
                                }
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
                if !*self.game_engine.log_engine.halted.borrow() {
                    self.game_engine.update();
                    let pixels = self.game_engine.pixels();

                    //Hydrate the screen based on scaling factors and stuff
                    let pixels_rc = pixels.borrow();
                    let height = pixels_rc.len();
                    let width = pixels_rc[0].len();
                    for y in 0..height{
                        for x in 0..width{
                            for dy in 0..SCALE{
                                for dx in 0..SCALE{
                                    let idx = (y * SCALE + dy) * WINDOW_WIDTH as usize + (x * SCALE + dx);
                                    let COLORS(r, g, b, a) = pixels_rc[y][x];
                                    buffer[idx*4..idx*4+4].copy_from_slice(&[r, g, b, a]);
                                }
                            }
                        }
                    }
                }

                self.game_engine.log_engine.update();
                let pixels = self.game_engine.log_engine.pixels();
                
                let pixels_rc = pixels.borrow();
                let height = pixels_rc.len();
                let width = pixels_rc[0].len();
                for y in 0..height{
                    for x in 0..width{
                        for dy in 0..SCALE{
                            for dx in 0..SCALE{
                                let idx = (y * SCALE + dy) * WINDOW_WIDTH + (x * SCALE + dx) + WINDOW_WIDTH * SCREEN_SIZE * SCALE;
                                let COLORS(r, g, b, a) = pixels_rc[y][x];
                                buffer[idx*4..idx*4+4].copy_from_slice(&[r, g, b, a]);
                            }
                        }
                    }
                }

                if self.game_engine.log_engine.restart {
                    self.game_engine = GameEngine::new()?;
                }

                Ok(())
            }
        }
    }
}
