use std::{cell::RefCell, collections::HashSet, rc::Rc, usize};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
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

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match engine_rc.borrow().state_engine{
                                StateEngines::GameEngine => {
                                    bind_keyboard(engine_rc.borrow().game_engine.keys_pressed.clone(), engine_rc.borrow().game_engine.keys_just_pressed.clone(), input.state, keycode);
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
                                bind_mouse_input(engine_rc.borrow().game_engine.mouse.clone(), button, state);
                                bind_mouse_input(engine_rc.borrow().game_engine.log_engine.mouse.clone(), button, state);
                            }
                        };
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        match engine_rc.borrow().state_engine {
                            StateEngines::GameEngine => {
                                let scale = window.scale_factor();
                                let logical = position.to_logical::<f32>(scale);

                                let mouse_rc = engine_rc.borrow().game_engine.mouse.clone();
                                bind_mouse_move(mouse_rc.clone(), logical, 0, 0, WINDOW_WIDTH, WINDOW_WIDTH);
                                let mouse_rc = engine_rc.borrow().game_engine.log_engine.mouse.clone();
                                bind_mouse_move(mouse_rc, logical, 0, WINDOW_WIDTH, WINDOW_WIDTH, WINDOW_WIDTH);
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

                    copy_pixels_into_buffer(pixels, buffer, 0, 0);
                }

                self.game_engine.log_engine.update();
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
fn copy_pixels_into_buffer(pixels: Rc<RefCell<PixelsType>>, buffer: &mut [u8], start_x: usize, start_y: usize){
    let pixels_rc = pixels.borrow();
    let height = pixels_rc.len();
    let width = pixels_rc[0].len();
    for y in 0..height{
        for x in 0..width{
            for dy in 0..SCALE{
                for dx in 0..SCALE{
                    let idx = ((y * SCALE + dy) * WINDOW_WIDTH as usize + (x * SCALE + dx)) + start_y * WINDOW_WIDTH + start_x;
                    let COLORS(r, g, b, a) = pixels_rc[y][x];
                    buffer[idx*4..idx*4+4].copy_from_slice(&[r, g, b, a]);
                }
            }
        }
    }
}

fn bind_keyboard(keys_pressed: Rc<RefCell<HashSet<VirtualKeyCode>>>, keys_just_pressed: Rc<RefCell<HashSet<VirtualKeyCode>>>, state: ElementState, keycode: VirtualKeyCode){
    match state {
        ElementState::Pressed => {
            if !keys_pressed.borrow().contains(&keycode) {
                keys_just_pressed.borrow_mut().insert(keycode);
            }
            keys_pressed.borrow_mut().insert(keycode);
        }
        ElementState::Released => {
            keys_pressed.borrow_mut().remove(&keycode);
        }
    }
}

fn bind_mouse_input(mouse: Rc<RefCell<MousePress>>, button: MouseButton, state: ElementState){
    if button == MouseButton::Left {
        match state {
            ElementState::Pressed => {
                mouse.borrow_mut().pressed = true;
                mouse.borrow_mut().just_pressed = true;
            },
            ElementState::Released => {
                mouse.borrow_mut().pressed = false;
                mouse.borrow_mut().just_pressed = false;
            },
        }
    }
}

fn check_mouse_bounds(mouse: Rc<RefCell<MousePress>>, start_x: usize, start_y: usize, width: usize, height: usize) -> bool {
    let cur_x = mouse.borrow().x as usize;
    let cur_y = mouse.borrow().y as usize;
    if cur_x < start_x || cur_x > start_x + width || cur_y < start_y || cur_y > start_y + height {
        mouse.borrow_mut().pressed = false;
        mouse.borrow_mut().just_pressed = false;
        mouse.borrow_mut().x = -1;
        mouse.borrow_mut().y = -1;
        return true;
    }

    false
}

fn bind_mouse_move(mouse: Rc<RefCell<MousePress>>, logical_position: LogicalPosition<f32>, start_x: usize, start_y: usize, width: usize, height: usize){
    mouse.borrow_mut().x = logical_position.x as i32;
    mouse.borrow_mut().y = logical_position.y as i32;

    if !check_mouse_bounds(mouse.clone(), start_x, start_y, width, height){
        mouse.borrow_mut().x -= start_x as i32;
        mouse.borrow_mut().y -= start_y as i32;

        mouse.borrow_mut().x /= SCALE as i32;
        mouse.borrow_mut().y /= SCALE as i32;
    };
}
