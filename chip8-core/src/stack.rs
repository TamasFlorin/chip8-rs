const STACK_SIZE: usize = 16;

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
