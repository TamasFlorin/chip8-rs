pub const NUM_KEYS: usize = 16;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KeyState {
    Up,
    Down,
}

pub type Keys = [KeyState; NUM_KEYS];