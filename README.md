# RICO-32

<div align="center">

**The Rust Imaginary Console Offline**

A fantasy console inspired by PICO-8, built with Rust for performance and Lua for scripting.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Screenshots](#screenshots)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [TODO](#todo)

## Introduction

RICO-32 is a fantasy console that combines the nostalgic charm of retro game development with modern Rust performance. It features a 128×128 pixel game screen with a 16-color palette, Lua scripting support, and a built-in console for debugging and logging.

The console is designed to be simple yet powerful, allowing developers to create games quickly using Lua while benefiting from Rust's performance and safety guarantees. Whether you're prototyping a game idea or creating a full retro-style game, RICO-32 provides the tools you need.

## Features

- **128×128 Pixel Display**: Classic retro resolution with 4× pixel scaling for modern displays
- **16-Color Palette**: Predefined color palette for consistent retro aesthetics
- **Lua Scripting**: Full Lua 5.4 support with custom module system
- **Built-in Console**: Integrated console engine for logging and debugging
- **Image Support**: Load and draw PNG images from the assets directory
- **Input Handling**: Mouse and keyboard input with frame-accurate state tracking
- **Frame Rate Control**: Configurable frame rate with delta time support
- **Modular Architecture**: Clean separation between game engine, console engine, and scripting
- **Hot Reload Support**: Restart functionality for rapid iteration

## Screenshots

### GIF of Tetris Example
![ezgif-66742301f2ba23f0](https://github.com/user-attachments/assets/d14c9ba0-433c-43ab-841a-33a669e46b75)

**more examples eventually...**


## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Cargo (comes with Rust)
- A C compiler (for building native dependencies) (i think)

### Building from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/RICO-32/RICO-32.git
   cd RICO-32
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the project**:
   ```bash
   cargo run
   ```

   Or run the release binary directly:
   ```bash
   cargo run --release
   ```

## Quick Start

1. **Create your game script**: Edit `scripts/main.lua` to start building your game.

2. **Add assets** (optional): Place PNG images in the `assets/` directory.

3. **Run your game**: Execute `cargo run` and your game will start!

### Example Game

```lua
function start()
    rico:log("Welcome to RICO-32!")
    rico:set_frame_rate(60)
end

function update(dt)
    rico:clear("BLACK")
    rico:print_scr(10, 10, "WHITE", "Hello, World!")
    
    local mouse = rico:mouse()
    if mouse.pressed then
        rico:circle(mouse.x, mouse.y, 5, "RED")
    end
end
```

## API Reference

All RICO API functions must be prefixed with `rico:` to be called in Lua.

### Core Functions

#### `start()`
Called once at the start of the program. Use this for initialization.

#### `update(dt)`
Called every frame. `dt` is the delta time in milliseconds since the last frame.

### Graphics Functions

#### `set_pix(x, y, COLOR)`
Sets the pixel at coordinates `(x, y)` to the specified color.

**Parameters:**
- `x` (number): X coordinate (0-127)
- `y` (number): Y coordinate (0-127)
- `COLOR` (string): One of the 16 color names

#### `get_pix(x, y) -> COLOR`
Gets the color of the pixel at coordinates `(x, y)`.

**Returns:** Color name as a string

#### `clear(COLOR)`
Fills the entire screen with the specified color.

#### `rectfill(x, y, w, h, COLOR)`
Fills a rectangle with the specified color.

**Parameters:**
- `x, y` (number): Top-left corner coordinates
- `w, h` (number): Width and height
- `COLOR` (string): Fill color

#### `rect(x, y, w, h, COLOR)`
Draws a rectangle outline with the specified color.

#### `circle(x, y, r, COLOR)`
Draws a filled circle.

**Parameters:**
- `x, y` (number): Center coordinates
- `r` (number): Radius
- `COLOR` (string): Fill color

#### `draw(x, y, file)`
Draws an image from the assets directory.

**Parameters:**
- `x, y` (number): Top-left corner coordinates
- `file` (string): Filename in the `assets/` directory (e.g., `"sprite.png"`)

### Text Functions

#### `print_scr(x, y, COLOR, text)`
Prints text to the screen. Each character is 8×8 pixels.

#### `print_scr_mid(x, y, COLOR, text)`
Prints text with medium-sized characters (4×6 pixels).

#### `print_scr_mini(x, y, COLOR, text)`
Prints text with mini-sized characters (4×4 pixels).

### Input Functions

#### `mouse() -> mouse_object`
Returns a mouse object with the following properties:
- `.just_pressed` (boolean): Whether the left button was just pressed (lasts 1 frame)
- `.pressed` (boolean): Whether the left button is currently pressed
- `.x` (number): X coordinate in pixels (-1 if outside window)
- `.y` (number): Y coordinate in pixels (-1 if outside window)

#### `key_pressed(NAME) -> boolean`
Returns whether the specified key is currently pressed.

#### `key_just_pressed(NAME) -> boolean`
Returns whether the specified key was just pressed this frame.

**Supported Keys:**
- Numbers: `"1"` through `"0"`
- Letters: `"A"` through `"Z"`
- Arrows: `"Left"`, `"Up"`, `"Right"`, `"Down"`
- Special: `"Back"`, `"Enter"`, `"Space"`

### System Functions

#### `log(message)`
Prints a message to the console. Messages are displayed in the console panel below the game screen.

#### `set_frame_rate(rate)`
Sets the target frame rate. Set to 0 or negative for unlimited frame rate.

### Colors

The following 16 colors are available:

- `"BLACK"`, `"WHITE"`, `"RED"`, `"LIME"`
- `"BLUE"`, `"YELLOW"`, `"CYAN"`, `"MAGENTA"`
- `"SILVER"`, `"GRAY"`, `"MAROON"`, `"OLIVE"`
- `"GREEN"`, `"PURPLE"`, `"TEAL"`, `"NAVY"`

## Examples

The project includes several example games in the `scripts/examples/` directory:

- **Platformer**: A simple platformer game
- **Shooter**: A top-down shooter example
- **Tetris**: Classic Tetris implementation

To run an example, copy its contents to `scripts/main.lua` or modify `scripts/main.lua` to require the example module.

## Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**: Follow Rust and Lua best practices
4. **Commit your changes**: `git commit -m 'Add amazing feature'`
5. **Push to the branch**: `git push origin feature/amazing-feature`
6. **Open a Pull Request**: Provide a clear description of your changes

### Development Guidelines

- Follow Rust naming conventions and style guidelines
- Add comments for complex logic
- Update documentation for new API functions
- Test your changes with example games
- Ensure the project builds with `cargo build`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Dhruv Goel

## TODO

### Planned Features

- [ ] **Sprite Engine**: Develop a comprehensive sprite engine with support for:
  - Sprite sheets and animations
  - Sprite flipping and rotation
  - Sprite collision detection
  - Sprite batching for performance optimization
  
- [ ] **Integrated Development Environment (IDE)**: Create a built-in IDE featuring:
  - Code editor with syntax highlighting for Lua
  - Live code reloading
  - Integrated asset browser
  - Debugging tools and breakpoints
  - Performance profiler
  - Visual sprite editor

### Future Enhancements

- [ ] Sound and music support
- [ ] Save/load game state functionality
- [ ] Export to web (WebAssembly)
- [ ] Additional graphics primitives (lines, polygons)
- [ ] Tilemap support
- [ ] Physics engine integration
- [ ] Networking capabilities for multiplayer games

---

<div align="center">

Made with <3

[Report Bug](https://github.com/RICO-32/RICO-32/issues) · [Request Feature](https://github.com/RICO-32/RICO-32/issues)

</div>
