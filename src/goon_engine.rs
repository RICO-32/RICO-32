use std::{cell::RefCell, rc::Rc};

use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::game_engine::{GameEngine};

const SCREEN_SIZE: u32 = 128;

/* All screen engines must implement
 * Game for now, sprite in the future, maybe IDE
 */
pub trait ScreenEngine {
    fn update(&mut self) -> LuaResult<()>;    
}

/* Add bindings for diff engines in this struct
 * All engines are different screens on the console
 * Engines should implement the Engine trait
 */
pub struct GoonEngine{
    buffer: Vec<u8>,
    game_engine: GameEngine,
    state: i32
}

impl GoonEngine{
    pub fn new() -> LuaResult<Self>{
        let engine = GoonEngine{
            buffer: vec![0u8; (SCREEN_SIZE * SCREEN_SIZE * 4) as usize],
            game_engine: GameEngine::new()?,
            state: 0
       };

        Ok(engine)
    }

    pub fn start(mut self) -> Result<(), Box<dyn std::error::Error>>{
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("The Goon Engine")
            .with_inner_size(LogicalSize::new(SCREEN_SIZE as f64, SCREEN_SIZE as f64))
            .build(&event_loop)?;

        let surface_texture = SurfaceTexture::new(SCREEN_SIZE, SCREEN_SIZE, &window);
        let mut pixels = Pixels::new(SCREEN_SIZE, SCREEN_SIZE, surface_texture)?;

        for y in 0..SCREEN_SIZE {
            for x in 0..SCREEN_SIZE {
                let i = ((y * SCREEN_SIZE + x) * 4) as usize;
                self.buffer[i] = (x as u8).wrapping_mul(3);      // R
                self.buffer[i + 1] = (y as u8).wrapping_mul(2);  // G
                self.buffer[i + 2] = ((x ^ y) as u8).wrapping_mul(1); // B (simple xor pattern)
                self.buffer[i + 3] = 0xFF;                      // A
            }
        }

        let engine_rc = Rc::new(RefCell::new(self)).clone();
        // Event loop: Poll so we run as fast as possible and continuously request redraws.
        event_loop.run(move |event, _, control_flow| {
            // Poll loop -> render as fast as possible
            *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(_) => {
                let _ = engine_rc.borrow_mut().update();
                let frame = pixels.get_frame_mut();
                frame.copy_from_slice(&engine_rc.borrow().buffer);

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
    pub fn update(&mut self) -> LuaResult<()> {
        self.buffer[0] = 127;
        self.buffer[1] = 127;
        self.buffer[2] = 127;
        match self.state {
            0 => Ok(self.game_engine.update()?),
            _ => Ok(())
        }
    }
}
