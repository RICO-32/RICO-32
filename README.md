# GoonMachine

The ultimate pico clone because it's a cool ass idea

# Lua bindings for the GoonMachine

## Features
- Implemented modularization for vanilla Lua API
- Auto runs main.lua in /scripts
- A base screen that can maniluated from the API
- Modularized console engine

## TODO
- [ ] Add Sprite engine
- [ ] Add IDE engine

## API
- start() is called at the start of the program
- update(dt) is called every frame based on the frame rate and passed in the delta time for that frame in milliseconds
- log: Prints out a string in the console
- set_frame_rate: Sets the frame rate of the game and how often update is called
- set_pix(x, y, COLOR) sets the pixel at x, y to COLOR where color is one of the 16 strings defined
- get_pox(x, y) gets the pixel at x, y and returns with one of the 16 colors
- Placeholder draw/print_scr/button

## Things to note
- Do not call nested API funcs (Ex: log(get_pix(0, 0))), not supported for now

## License

The Goon Machine is made available under the MIT [License](./LICENSE).
