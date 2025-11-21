# GoonMachine

The ultimate pico clone because it's a cool ass idea

# Lua bindings for the GoonMachine

## Features
- Implemented modularization for vanilla Lua API
- Auto runs main.lua in /scripts

## API
- start() is called at the start of the program
- update(dt) is called every frame based on the frame rate and passed in the delta time for that frame in milliseconds
- log: Prints out a string in the console
- set_frame_rate: Sets the frame rate of the game and how often update is called
- Placeholder draw/print_scr/button

## License

The Goon Machine is made available under the MIT [License](./LICENSE).
