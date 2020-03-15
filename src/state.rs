const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
const PROGRAM_START: usize = 512;
const SPRITES_START: usize = 0;
const SPRITES_END: usize = 0x1FF;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
static SPRITE_0: &'static [u8] = &[0xF0, 0x90, 0x90, 0x90, 0xF0];
static SPRITE_1: &'static [u8] = &[0x20, 0x60, 0x20, 0x20, 0x70];
static SPRITE_2: &'static [u8] = &[0xF0, 0x10, 0xF0, 0x80, 0xF0];
static SPRITE_3: &'static [u8] = &[0xF0, 0x10, 0xF0, 0x10, 0xF0];
static SPRITE_4: &'static [u8] = &[0x90, 0x90, 0xF0, 0x10, 0x10];
static SPRITE_5: &'static [u8] = &[0xF0, 0x80, 0xF0, 0x10, 0xF0];
static SPRITE_6: &'static [u8] = &[0xF0, 0x80, 0xF0, 0x90, 0xF0];
static SPRITE_7: &'static [u8] = &[0xF0, 0x10, 0x20, 0x40, 0x40];
static SPRITE_8: &'static [u8] = &[0xF0, 0x90, 0xF0, 0x90, 0xF0];
static SPRITE_9: &'static [u8] = &[0xF0, 0x90, 0xF0, 0x10, 0xF0];
static SPRITE_A: &'static [u8] = &[0xF0, 0x90, 0xF0, 0x90, 0x90];
static SPRITE_B: &'static [u8] = &[0xE0, 0x90, 0xE0, 0x90, 0xE0];
static SPRITE_C: &'static [u8] = &[0xF0, 0x80, 0x80, 0x80, 0xF0];
static SPRITE_D: &'static [u8] = &[0xE0, 0x90, 0x90, 0x90, 0xE0];
static SPRITE_E: &'static [u8] = &[0xF0, 0x80, 0xF0, 0x80, 0xF0];
static SPRITE_F: &'static [u8] = &[0xF0, 0x80, 0xF0, 0x80, 0x80];
static FONT_SPRITES: &'static [&[u8]] = &[
    &SPRITE_0, &SPRITE_1, &SPRITE_2, &SPRITE_3, &SPRITE_4, &SPRITE_5, &SPRITE_6, &SPRITE_7,
    &SPRITE_8, &SPRITE_9, &SPRITE_A, &SPRITE_B, &SPRITE_C, &SPRITE_D, &SPRITE_E, &SPRITE_F,
];

pub struct Memory {
    buffer: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];

        // load program data
        for i in 0..program.len() {
            memory[i + PROGRAM_START] = program[i];
        }

        let mut i = 0;
        while i <= SPRITES_END {
            let current = FONT_SPRITES[i];
            for &value in current {
                memory[i] = value;
                i += 1;
            }
        }
        // load sprite data
        for i in SPRITES_START..SPRITES_END {
            memory[i] = FONT_SPRITES[i][i];
        }

        Memory { buffer: memory }
    }

    pub fn get(&self, index: usize) -> u8 {
        self.buffer[index]
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        let first = self.buffer[index] as u16;
        let second = self.buffer[index + 1] as u16;
        (first << 8) + second
    }

    pub fn set(&mut self, index: usize, value: u8) {
        self.buffer[index] = value;
    }
}

#[derive(Clone)]
pub struct Stack {
    buffer: [u16; STACK_SIZE],
    head: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            buffer: [0; STACK_SIZE],
            head: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.buffer[self.head] = value;
        self.head += 1
    }

    pub fn pop(&mut self) -> u16 {
        assert!(self.head > 0);
        self.head -= 1;
        self.buffer[self.head]
    }

    pub fn top(&self) -> u16 {
        assert!(self.head > 0);
        self.buffer[self.head - 1]
    }
}

#[derive(Clone)]
pub struct State {
    pub memory: [u8; MEMORY_SIZE],
    pub registers: [u8; NUM_REGISTERS],
    pub stack: Stack,
    pub display_buffer: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: usize, // program counter
    pub i: u16,    // register used to store memory addresses
    pub should_draw: bool,
    pub clear_display: bool,
}

impl State {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];
        for i in 0..program.len() {
            memory[i + PROGRAM_START] = program[i];
        }

        State {
            memory,
            registers: [0; NUM_REGISTERS],
            stack: Stack::new(),
            display_buffer: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            pc: PROGRAM_START,
            i: 0,
            should_draw: false,
            clear_display: false,
        }
    }

    pub fn instruction(&self) -> u16 {
        debug_assert!(self.pc < self.memory.len());
        let current_pc = self.pc as usize;
        let first = self.memory[current_pc] as u16;
        let second = self.memory[current_pc + 1] as u16;
        (first << 8) + second
    }

    pub fn next_instruction(&mut self) {
        self.pc += 2;
    }

    pub fn set_register(&mut self, reg_index: usize, reg_value: u8) {
        debug_assert!(reg_index < self.registers.len());
        self.registers[reg_index] = reg_value;
    }

    pub fn set_address_register(&mut self, address: u16) {
        debug_assert!(address < self.memory.len() as u16);
        self.i = address;
    }
}
