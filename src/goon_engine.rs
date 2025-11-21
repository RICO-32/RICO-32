use std::{cell::RefCell, rc::Rc, usize};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::game_engine::{GameEngine};
use crate::colors::COLORS;

pub const SCREEN_SIZE: u32 = 128;
pub const WINDOW_SIZE: u32 = SCREEN_SIZE * 1;

pub type PixelsType = [[COLORS; SCREEN_SIZE as usize]; SCREEN_SIZE as usize];

/* All screen engines must implement
 * Game for now, sprite in the future, maybe IDE
 */
pub trait ScreenEngine {
    fn pixels(&self) -> Rc<RefCell<PixelsType>>;
    fn update(&mut self) -> LuaResult<()>;    
}

/* Add bindings for diff engines in this struct
 * All engines are different screens on the console
 * Engines should implement the Engine trait
 */
pub struct GoonEngine{
    game_engine: GameEngine,
    state: i32
}

impl GoonEngine{
    pub fn new() -> LuaResult<Self>{
        let engine = GoonEngine{
            game_engine: GameEngine::new()?,
            state: 0
        };

        Ok(engine)
    }

    pub fn start(self) -> Result<(), Box<dyn std::error::Error>>{
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("The Goon Engine")
            .with_inner_size(LogicalSize::new(WINDOW_SIZE as f64, WINDOW_SIZE as f64))
            .build(&event_loop)?;

        let surface_texture = SurfaceTexture::new(WINDOW_SIZE, WINDOW_SIZE, &window);
        let mut pixels = Pixels::new(WINDOW_SIZE, WINDOW_SIZE, surface_texture)?;

        let engine_rc = Rc::new(RefCell::new(self)).clone();
        // Event loop: Poll so we run as fast as possible and continuously request redraws.
        event_loop.run(move |event, _, control_flow| {
            // Poll loop -> render as fast as possible
            *control_flow = ControlFlow::Poll;

            match event {
                Event::RedrawRequested(_) => {
                    let frame = pixels.get_frame_mut();
                    let _ = engine_rc.borrow_mut().update(frame);

                    if let Err(_) = pixels.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                Event::MainEventsCleared => {
                    window.request_redraw();
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. }, ..
                } => {
                    if let Some(winit::event::VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {}
            }
        });
    }

    //Make sure to update engines here based on which screen it's on
    pub fn update(&mut self, buffer: &mut [u8]) -> LuaResult<()> {
        let pixels = match self.state {
            0 => {
                self.game_engine.update()?;
                self.game_engine.pixels()
            },
            _ => Rc::from(RefCell::from(COLORS::pixels()))
        };

        let pixels_rc = pixels.borrow();
        let scale = (WINDOW_SIZE / SCREEN_SIZE) as usize;
        for y in 0..SCREEN_SIZE as usize{
            for x in 0..SCREEN_SIZE as usize{
                for dy in 0..scale{
                    for dx in 0..scale{
                        let idx = (y * scale + dy) * WINDOW_SIZE as usize + (x * scale + dx);
                        let COLORS(r, g, b) = pixels_rc[y][x];
                        buffer[idx*4..idx*4+4].copy_from_slice(&[r, g, b, 0xFF]);
                    }
                }
            }
        }

        Ok(())
    }
}
