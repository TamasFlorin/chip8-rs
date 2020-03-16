
pub use crate::state::DISPLAY_HEIGHT;
pub use crate::state::DISPLAY_WIDTH;

pub trait Display {
    fn draw(&mut self, pixels: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}