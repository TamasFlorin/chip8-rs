# Chip8-rs :crab:

This is my first attempt at implementing the Chip-8 interpreted programming language in Rust.\
The implementation was mostly done by following [Cowgod's technical reference.](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

![pong_demo](https://user-images.githubusercontent.com/8793421/76801346-23d84c80-67de-11ea-8bf1-661372c03390.gif)

# Installation
The project requires sdl2_gfx to be installed locally.\
On MacOs the following command will install the required dependency:\
`brew install sdl2_gfx`

Afterwards, you can just run:\
`cd cargo-sdl`
`cargo run --release path_to_rom_file`

# Dependencies
`rand = "0.7.3"`\
`sdl2 = "0.33.0"`
