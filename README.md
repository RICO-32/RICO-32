# GoonMachine

The ultimate pico clone because it's a cool ass idea

# GoonLang

Lua bindings for the GoonMachine

## Features
- Implemented modularization for vanilla Lua API
- Auto runs main.lua in /scripts
- Implements sprites and exposes them to lua
    - the draw function is defined and called each frame for every sprite

## API
- start() is called at the start of the program
- update() is called every frame based on the frame rate
- log: Prints out a string in the console
- set_frame_rate: Sets the frame rate of the game and how often update is called
- Sprite is a struct exposed
    - Sprite.new(file, x, y, size) to make a new Sprite
    - Getters and setters for all fields

## License

The Goon Machine is made available under the MIT [License](./LICENSE).
