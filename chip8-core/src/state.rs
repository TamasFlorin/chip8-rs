use crate::ram::{Ram, PROGRAM_START};
use crate::stack::Stack;

const NUM_REGISTERS: usize = 16;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

#[derive(Clone)]
pub struct State {
    pub ram: Ram,
    pub registers: [u8; NUM_REGISTERS],
    pub stack: Stack,
    pub display_buffer: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: usize, // program counter
    pub i: u16,    // register used to store memory addresses
    pub should_draw: bool,
    pub waiting_for_key: bool,
    pub key_register_index: usize,
    pub play_audio: bool,
}

impl State {
    pub fn new(program: &[u8]) -> Self {
        State {
            ram: Ram::new(program),
            registers: [0; NUM_REGISTERS],
            stack: Stack::new(),
            display_buffer: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            pc: PROGRAM_START,
            i: 0,
            should_draw: false,
            waiting_for_key: false,
            key_register_index: 0,
            play_audio: false,
        }
    }

    pub fn instruction(&self) -> u16 {
        self.ram.get_u16(self.pc)
    }

    pub fn next_instruction(&mut self) {
        self.pc += 2;
    }

    pub fn set_address_register(&mut self, address: u16) {
        debug_assert!(address < self.ram.len() as u16);
        self.i = address;
    }
}
