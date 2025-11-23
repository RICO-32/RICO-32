use std::{cell::RefCell, collections::HashSet, rc::Rc, usize};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{game_engine::GameEngine, utils::mouse::MousePress};
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
    fn update(&mut self) -> LuaResult<()>;    
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
    mouse: MousePress,
    keys_pressed: HashSet<VirtualKeyCode>
}

impl RicoEngine{
    pub fn new() -> LuaResult<Self>{
        let engine = RicoEngine{
            game_engine: GameEngine::new()?,
            state_engine: StateEngines::GameEngine,
            mouse: MousePress::default(),
            keys_pressed: HashSet::new()
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
                            match input.state {
                                winit::event::ElementState::Pressed => {
                                    engine_rc.borrow_mut().keys_pressed.insert(keycode);
                                }
                                winit::event::ElementState::Released => {
                                    engine_rc.borrow_mut().keys_pressed.remove(&keycode);
                                }
                            }

                            // Example: exit on ESC
                            if keycode == winit::event::VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    },

                    WindowEvent::MouseInput { button, state, .. } => {
                        match state {
                            winit::event::ElementState::Pressed => {
                                if button == MouseButton::Left {
                                    engine_rc.borrow_mut().mouse.pressed = true;
                                }
                            }
                            winit::event::ElementState::Released => {
                                if button == MouseButton::Left {
                                    engine_rc.borrow_mut().mouse.pressed = false;
                                }
                            }
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let scale = window.scale_factor();
                        let logical = position.to_logical::<f32>(scale);

                        let mut eng = engine_rc.borrow_mut();
                        eng.mouse.x = logical.x as i32;
                        eng.mouse.y = logical.y as i32;
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
                let mut new_press = MousePress { pressed: self.mouse.pressed, x: -1, y: -1  };

                if self.mouse.x as usize / SCALE <= SCREEN_SIZE && self.mouse.y as usize / SCALE <= SCREEN_SIZE {
                    new_press = MousePress { pressed: self.mouse.pressed, x: self.mouse.x / SCALE as i32, y: self.mouse.y / SCALE as i32  };
                }

                *self.game_engine.mouse.borrow_mut() = new_press;
                *self.game_engine.keys_pressed.borrow_mut() = self.keys_pressed.clone();
                self.game_engine.update()?;
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

                self.game_engine.log_engine.update()?;
                let pixels = self.game_engine.log_engine.pixels();
                
                //Hydrate the screen based on scaling factors and stuff
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
                Ok(())
            }
        }
    }
}
