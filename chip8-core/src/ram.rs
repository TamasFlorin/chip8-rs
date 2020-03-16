const MEMORY_SIZE: usize = 4096;
pub const PROGRAM_START: usize = 512;

static FONT_SPRITES: &'static [u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Clone)]
pub struct Ram {
    buffer: [u8; MEMORY_SIZE],
}

impl Ram {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];

        // load program data
        for i in 0..program.len() {
            memory[i + PROGRAM_START] = program[i];
        }

        for i in 0..FONT_SPRITES.len() {
            memory[i] = FONT_SPRITES[i];
        }

        Self { buffer: memory }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
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
