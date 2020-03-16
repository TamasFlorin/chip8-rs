
pub use chip8::display::Display;
use chip8::display::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use sdl2::gfx::framerate::FPSManager;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = DISPLAY_WIDTH as u32 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = DISPLAY_HEIGHT as u32 * SCALE_FACTOR;

const WHITE: Color = Color::RGB(255, 255, 255);
const BLACK: Color = Color::RGB(0, 0, 0);
const FRAME_RATE: u32 = 60;

pub struct SdlDisplay {
    canvas: Canvas<Window>,
    fps_manager: FPSManager,
}

impl SdlDisplay {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip8-rs", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(FRAME_RATE).unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        SdlDisplay {
            canvas,
            fps_manager,
        }
    }
}

impl Display for SdlDisplay {
    fn draw(&mut self, pixels: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        for y in 0..pixels.len() {
            for x in 0..pixels[0].len() {
                if pixels[y][x] == 1 {
                    self.canvas.set_draw_color(WHITE);
                } else {
                    self.canvas.set_draw_color(BLACK);
                }
                let x = x as i32 * SCALE_FACTOR as i32;
                let y = y as i32 * SCALE_FACTOR as i32;
                self.canvas
                    .fill_rect(Rect::new(x, y, SCALE_FACTOR, SCALE_FACTOR))
                    .unwrap();
            }
        }
        self.canvas.present();
        self.fps_manager.delay();
    }
}
