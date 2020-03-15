use ::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::{Read, Result};
use std::time::Duration;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = 64 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = 32 * SCALE_FACTOR;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("chip8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let rom_file_path = "roms/pong.ch8";
    let program = read_program(rom_file_path).unwrap();
    let mut computer = Chip8::new(program.as_slice());

    'running: loop {
        //canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        match computer.iteration() {
            Some(state) => {
                if state.clear_display {
                    canvas.clear();
                }
                if state.should_draw {
                    for y in 0..state.display_buffer.len() {
                        for x in 0..state.display_buffer[0].len() {
                            if state.display_buffer[y][x] == 1 {
                                canvas.set_draw_color(Color::RGB(255, 255, 255));
                            }
                            else {
                                canvas.set_draw_color(Color::RGB(0, 0, 0));
                            }
                            let x = x as i32 * SCALE_FACTOR as i32;
                            let y = y as i32 * SCALE_FACTOR as i32;
                            canvas
                            .draw_rect(Rect::new(x, y, SCALE_FACTOR, SCALE_FACTOR))
                            .unwrap();
                        }
                    }
                }
            }
            None => break,
        };
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
}

fn read_program(rom_file_path: &str) -> Result<Vec<u8>> {
    let mut rom_file = File::open(rom_file_path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    match rom_file.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}
