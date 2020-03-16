use chip8::Chip8;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Result};
use chip8_sdl::display::{Display, SdlDisplay};
use chip8_sdl::keyboard::{Keyboard, SdlKeyboard};
use chip8_sdl::audio::{Audio, SdlAudio};

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if let Some(rom_file_path) = arguments.get(0) {
        let program = read_program(rom_file_path).unwrap();
        let mut computer = Chip8::new(program.as_slice());
        let sdl_context = sdl2::init().unwrap();
        let mut display = SdlDisplay::new(&sdl_context);
        let mut keyboard = SdlKeyboard::new(&sdl_context);
        let audio = SdlAudio::new(&sdl_context);
        while let Ok(keys) = keyboard.poll() {
            match computer.iteration(&keys) {
                Some(state) => {
                    if state.should_draw {
                        display.draw(&state.display_buffer);
                    }
                    if state.play_audio {
                        audio.play();
                    } else {
                        audio.stop();
                    }
                }
                None => break,
            };
        }
    } else {
        println!("usage: chip8 rom_path");
    }
}

fn read_program<P: AsRef<Path>>(rom_file_path: P) -> Result<Vec<u8>> {
    let mut rom_file = File::open(rom_file_path.as_ref()).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    match rom_file.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}