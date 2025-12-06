use std::{ops::Deref, time::Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use super::{game::GameEngine, nav_bar::NavEngine, sprite::SpriteEngine};
use crate::{
    input::{keyboard::Keyboard, mouse::MousePress},
    render::colors::Colors,
};

pub const SCREEN_SIZE: usize = 128;
pub const SCALE: usize = 4;
pub const NAV_BAR_HEIGHT: usize = 8;
pub const WINDOW_WIDTH: usize = SCREEN_SIZE * SCALE;
pub const WINDOW_HEIGHT: usize = (NAV_BAR_HEIGHT + SCREEN_SIZE * 2) * SCALE;

pub type PixelsType = Vec<Vec<Colors>>;

/* All screen engines must implement
 * Game for now, sprite in the future, maybe IDE
 */
pub trait ScreenEngine {
    type Pixels<'a>: Deref<Target = PixelsType>
    where
        Self: 'a;
    fn pixels(&self) -> Self::Pixels<'_>;
}

enum StateEngines {
    GameEngine(GameEngine),
    SpriteEngine(Box<SpriteEngine>),
}

/* Add bindings for diff engines in this struct
 * All engines are different screens on the console
 * Engines should implement the Engine trait
 */
pub struct RicoEngine {
    nav_engine: NavEngine,
    state_engines: Vec<StateEngines>,
}

impl Default for RicoEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RicoEngine {
    pub fn new() -> Self {
        let game_eng = GameEngine::default();
        let sprite_eng = SpriteEngine::default();
        let state_engines = vec![
            StateEngines::GameEngine(game_eng),
            StateEngines::SpriteEngine(Box::new(sprite_eng)),
        ];

        RicoEngine {
            nav_engine: NavEngine::new(vec!["Game".to_string(), "Sprite".to_string()]),
            state_engines,
        }
    }

    //Base boot function, needs to take in whole self cause borrowing bs
    pub fn start(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("RICO-32")
            .with_resizable(false)
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64))
            .build(&event_loop)?;

        let surface_texture =
            SurfaceTexture::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, &window);
        let mut pixels = Pixels::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, surface_texture)?;

        // Event loop: Poll so we run as fast as possible and continuously request redraws
        event_loop.run(move |event, _, control_flow| {
            // Poll loop -> render as fast as possible
            *control_flow = ControlFlow::Poll;

            match event {
                Event::RedrawRequested(_) => {
                    //Pass in buffer and redraw all based pixels every frame
                    let buffer = pixels.frame_mut();
                    self.update(buffer);

                    if pixels.render().is_err() {
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
                            match self.state_engines[self.nav_engine.selected] {
                                StateEngines::GameEngine(ref mut eng) => {
                                    let mut lua_api = eng.lua_api.borrow_mut();
                                    bind_keyboard(&mut lua_api.keyboard, input.state, keycode);
                                }
                                StateEngines::SpriteEngine(ref mut eng) => {
                                    bind_keyboard(&mut eng.keyboard, input.state, keycode);
                                }
                            }

                            // exit on ESC
                            if keycode == winit::event::VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    }

                    WindowEvent::MouseWheel { delta, .. } => {
                        if let StateEngines::SpriteEngine(ref mut eng) =
                            self.state_engines[self.nav_engine.selected]
                        {
                            let scroll_y = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y,
                                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                            };

                            eng.update_start_row(scroll_y);
                        }
                    }

                    WindowEvent::MouseInput { button, state, .. } => {
                        bind_mouse_input(&mut self.nav_engine.mouse, button, state);
                        match self.state_engines[self.nav_engine.selected] {
                            StateEngines::GameEngine(ref mut eng) => {
                                bind_mouse_input(
                                    &mut eng.lua_api.borrow_mut().mouse,
                                    button,
                                    state,
                                );
                                bind_mouse_input(&mut eng.console_engine.mouse, button, state);
                            }
                            StateEngines::SpriteEngine(ref mut eng) => {
                                bind_mouse_input(&mut eng.mouse, button, state);
                            }
                        };
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let scale = window.scale_factor();
                        let logical = position.to_logical::<f32>(scale);

                        bind_mouse_move(
                            &mut self.nav_engine.mouse,
                            logical,
                            0,
                            0,
                            WINDOW_WIDTH,
                            NAV_BAR_HEIGHT * SCALE,
                        );
                        match self.state_engines[self.nav_engine.selected] {
                            StateEngines::GameEngine(ref mut eng) => {
                                bind_mouse_move(
                                    &mut eng.lua_api.borrow_mut().mouse,
                                    logical,
                                    0,
                                    NAV_BAR_HEIGHT * SCALE,
                                    WINDOW_WIDTH,
                                    WINDOW_WIDTH,
                                );
                                bind_mouse_move(
                                    &mut eng.console_engine.mouse,
                                    logical,
                                    0,
                                    NAV_BAR_HEIGHT * SCALE + WINDOW_WIDTH,
                                    WINDOW_WIDTH,
                                    WINDOW_WIDTH,
                                );
                            }
                            StateEngines::SpriteEngine(ref mut eng) => {
                                bind_mouse_move(
                                    &mut eng.mouse,
                                    logical,
                                    0,
                                    NAV_BAR_HEIGHT * SCALE,
                                    WINDOW_WIDTH,
                                    WINDOW_WIDTH * 2,
                                );
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
    pub fn update(&mut self, buffer: &mut [u8]) {
        self.nav_engine.update();
        let pixels = self.nav_engine.pixels();
        copy_pixels_into_buffer(pixels, buffer, 0, 0);
        match self.state_engines[self.nav_engine.selected] {
            StateEngines::GameEngine(ref mut eng) => {
                if self.nav_engine.just_switched {
                    eng.console_engine.last_time = Instant::now();
                }

                eng.update();

                let pixels = eng.pixels();
                copy_pixels_into_buffer(pixels.deref(), buffer, 0, NAV_BAR_HEIGHT * SCALE);

                let pixels = eng.console_engine.pixels();
                copy_pixels_into_buffer(pixels, buffer, 0, WINDOW_WIDTH + (NAV_BAR_HEIGHT * SCALE));
            }
            StateEngines::SpriteEngine(ref mut eng) => {
                eng.update();
                let pixels = eng.pixels();

                copy_pixels_into_buffer(pixels, buffer, 0, NAV_BAR_HEIGHT * SCALE);
            }
        }
    }
}

//Hydrate the screen based on scaling factors and stuff
fn copy_pixels_into_buffer(pixels: &PixelsType, buffer: &mut [u8], start_x: usize, start_y: usize) {
    let height = pixels.len();
    let width = pixels[0].len();
    for (y, row) in pixels.iter().enumerate().take(height) {
        for (x, pix) in row.iter().enumerate().take(width) {
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let idx = ((y * SCALE + dy) * WINDOW_WIDTH + (x * SCALE + dx))
                        + start_y * WINDOW_WIDTH
                        + start_x;
                    let (r, g, b, a) = pix.rgba();
                    buffer[idx * 4..idx * 4 + 4].copy_from_slice(&[r, g, b, a]);
                }
            }
        }
    }
}

fn bind_keyboard(keyboard: &mut Keyboard, state: ElementState, keycode: VirtualKeyCode) {
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

/* IMPORTANT:
 * This function does not automatically fix just_pressed every frame.
 * Either do that in rico's update or the update of the engine that holds the mouse, see
 * game_engine.rs for example.
 */
fn bind_mouse_input(mouse: &mut MousePress, button: MouseButton, state: ElementState) {
    if button == MouseButton::Left {
        match state {
            ElementState::Pressed => {
                mouse.pressed = true;
                mouse.just_pressed = true;
            }
            ElementState::Released => {
                mouse.pressed = false;
                mouse.just_pressed = false;
            }
        }
    }
}

fn check_mouse_bounds(
    mouse: &mut MousePress,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> bool {
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

fn bind_mouse_move(
    mouse: &mut MousePress,
    logical_position: LogicalPosition<f32>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) {
    mouse.x = logical_position.x as i32;
    mouse.y = logical_position.y as i32;

    if !check_mouse_bounds(mouse, start_x, start_y, width, height) {
        mouse.x -= start_x as i32;
        mouse.y -= start_y as i32;

        mouse.x /= SCALE as i32;
        mouse.y /= SCALE as i32;
    };
}
