# RICO-32

The Rust Imaginary Console Offline

The ultimate pico clone in the objectively better language because it's a cool ass idea

# Lua bindings for RICO-32

## Features
- Implemented modularization for vanilla Lua API
- Auto runs main.lua in /scripts
- A base screen that can maniluated from the API
- Modularized console engine

## TODO
- [ ] Add Sprite engine
- [ ] Add IDE engine

## API
- `start()` is called at the start of the program
- `update(dt)` is called every frame based on the frame rate and passed in the delta time for that frame in milliseconds
- `log`: Prints out a string in the console
- `set_frame_rate`: Sets the frame rate of the game and how often update is called
- `set_pix(x, y, COLOR)` sets the pixel at `x, y` to `COLOR` where color is one of the 16 strings defined
- `get_pix(x, y)` gets the pixel at `x, y` and returns with one of the 16 colors
- `print_scr(x, y, COLOR, text)` prints the text to the screen at `x, y` 
    - 8x8 font with a gap of 1 pixel between each character automatically
- `print_scr_mini(x, y, COLOR, text)` prints the text to the screen at `x, y`
    - 4x4 font with a gap of 1 pixel between each character automatically
- `draw(x, y, file)` draws the image at `assets/file` at `x, y`
- `clear(COLOR)` fills the entire screen with the provided color
- `rectfill(x, y, w, h, COLOR)` fills a rectangle with that specific color
- `rect(x, y, w, h, COLOR)` creates a border rectangle with that specific color
- `mouse()` returns a mouse object with  
    - `.pressed` -> Whether the left button is currently pressed  
    - `.x`, `.y` -> the pixel coordinates of the mouse currently  
- `key_pressed(NAME)` returns a bool on whether that key is pressed that frame or not

## Colors 
- Accessing from Lua API is simply the string `"COLOR_NAME"`
- `"BLACK"`, `"WHITE"`, `"RED"`, `"LIME"`, `"BLUE"`, `"YELLOW"`, `"CYAN"`, `"MAGENTA"`, `"SILVER"`, `"GRAY"`, `"MAROON"`, `"OLIVE"`, `"GREEN"`, `"PURPLE"`, `"TEAL"`, `"NAVY"`


## Keyboard inputs
- Access whether a certain key is being pressed through `key_pressed("KEY_NAME")`
- Supported keys:

**Numbers:**  
`"1", "2", "3", "4", "5", "6", "7", "8", "9", "0"`

**Letters:**  
`"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"`

**Arrows:**  
`"Left", "Up", "Right", "Down"`

**Special:**  
`"Back", "Enter", "Space"`

## License
RICO-32 is made available under the MIT [License](./LICENSE).
