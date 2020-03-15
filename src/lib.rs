pub mod state;
use rand::prelude::*;
pub use state::State;

#[derive(Clone)]
pub struct Chip8 {
    pub state: State, // initial program state
}

impl Chip8 {
    pub fn new(program: &[u8]) -> Self {
        Chip8 {
            state: State::new(program),
        }
    }

    pub fn iteration(&mut self) -> Option<State> {
        let instruction = self.state.instruction();
        self.state.should_draw = false;
        self.state.clear_display = false;
        self.state.next_instruction();
        println!("Instruction={:X} at pc={}", instruction, self.state.pc);
        let new_state = match instruction & 0xF000 {
            0x0000 => match instruction & 0x00FF {
                0x00E0 => self._00e0(instruction),
                0x00EE => self._00ee(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            0x1000 => self._1nnn(instruction),
            0x2000 => self._2nnn(instruction),
            0x3000 => self._3xkk(instruction),
            0x4000 => self._4xkk(instruction),
            0x5000 => self._5xy0(instruction),
            0x6000 => self._6xkk(instruction),
            0x7000 => self._7xkk(instruction),
            0x8000 => match instruction & 0x000F {
                0x0000 => self._8xy0(instruction),
                0x0001 => self._8xy1(instruction),
                0x0002 => self._8xy2(instruction),
                0x0003 => self._8xy3(instruction),
                0x0004 => self._8xy4(instruction),
                0x0005 => self._8xy5(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            0xA000 => self._annn(instruction),
            0xC000 => self._cxkk(instruction),
            0xD000 => self._dxyn(instruction),
            0xE000 => match instruction & 0x00FF {
                0x00A1 => self._exa1(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            0xF000 => match instruction & 0x00FF {
                0x0007 => self._fx07(instruction),
                0x0015 => self._fx15(instruction),
                0x0018 => self._fx18(instruction),
                0x0033 => self._fx33(instruction),
                0x0065 => self._fx65(instruction),
                0x0029 => self._fx29(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            _ => panic!("Unknown opcode {:X}", instruction),
        };

        self.state = new_state.clone();
        Some(new_state)
    }

    pub fn run(&mut self) {
        loop {
            match self.iteration() {
                Some(state) => self.state = state,
                None => break,
            };
        }
    }

    /// Clear the display.
    fn _00e0(&self, _instruction: u16) -> State {
        let mut current_state = self.state.clone();
        current_state.clear_display = true;
        current_state
    }

    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn _00ee(&self, _instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let return_address = current_state.stack.pop();
        current_state.pc = return_address as usize;
        current_state
    }

    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn _1nnn(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let address = Self::_nnn(instruction);
        current_state.pc = address as usize;
        current_state
    }

    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. 
    /// The PC is then set to nnn.
    fn _2nnn(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let call_address = Self::_nnn(instruction);
        current_state.stack.push(current_state.pc as u16);
        current_state.pc = call_address as usize;
        current_state
    }

    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn _3xkk(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let vx = current_state.registers[register_index];
        if vx == value {
            current_state.next_instruction();
        }
        current_state
    }

    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn _4xkk(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let vx: u8 = current_state.registers[x];
        if vx != value {
            current_state.next_instruction();
        }
        current_state
    }

    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn _5xy0(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_x_index = Self::_x(instruction) as usize;
        let register_y_index = Self::_y(instruction) as usize;
        let vx = current_state.registers[register_x_index];
        let vy = current_state.registers[register_y_index];

        if vx == vy {
            current_state.next_instruction();
        }
        current_state
    }

    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn _6xkk(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let register_value = Self::_kk(instruction);
        current_state.set_register(register_index, register_value);
        current_state
    }

    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn _7xkk(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let register_value = current_state.registers[register_index] as u8;
        let add_result = register_value.overflowing_add(value).0;
        current_state.registers[register_index] = add_result;
        current_state
    }

    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn _8xy0(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        current_state.registers[x] = current_state.registers[y];
        current_state
    }

    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. 
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, 
    /// then the same bit in the result is also 1. Otherwise, it is 0. 
    fn _8xy1(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx = current_state.registers[x];
        let vy = current_state.registers[y];
        current_state.registers[x] = vx | vy;
        current_state
    }

    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn _8xy2(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx = current_state.registers[x];
        let vy = current_state.registers[y];
        current_state.registers[x] = vx & vy;
        current_state
    }

    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. 
    /// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, 
    /// then the corresponding bit in the result is set to 1. Otherwise, it is 0. 
    fn _8xy3(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx = current_state.registers[x];
        let vy = current_state.registers[y];
        current_state.registers[x] = vx ^ vy;
        current_state
    }

    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn _8xy4(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let result = current_state.registers[x] as u16 + current_state.registers[y] as u16;
        let result_u8 = (result & 0x00FF) as u8;
        current_state.registers[x] = result_u8;
        current_state.registers[0xF] = (result > std::u8::MAX as u16).into();
        current_state
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    fn _8xy5(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx: u8 = current_state.registers[x];
        let vy: u8 = current_state.registers[y];
        current_state.registers[x] = vx.overflowing_sub(vy).0;
        current_state.registers[0xF] = (vx > vy).into();
        current_state
    }

    /// Set I = nnn.
    /// The value of register I is set to nnn.
    fn _annn(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let address = Self::_nnn(instruction);
        current_state.set_address_register(address);
        current_state
    }

    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx.
    fn _cxkk(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let random_value: u8 = random();
        let final_value = random_value & value;
        current_state.registers[register_index] = final_value;
        current_state
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    fn _dxyn(&self, _instruction: u16) -> State {
        let mut current_state = self.state.clone();
        current_state.registers[0xF] = 0;
        let register_x = Self::_x(_instruction) as usize;
        let register_y = Self::_y(_instruction) as usize;

        let x_start = current_state.registers[register_x] as usize;
        let y_start = current_state.registers[register_y] as usize;
        let n = Self::_n(_instruction) as usize;
        let width = 8; // width of the sprite

        for y in 0..n {
            let address_register = current_state.i as usize;
            let current_pixel_index = address_register + y;
            for x in 0..width {
                let x_current = (x_start + x) % 64;
                let y_current = (y_start + y) % 32;
                let pixel: u8 = (current_state.memory[current_pixel_index] >> (7 - x)) & 1;
                current_state.registers[0xF] |= pixel & current_state.display_buffer[y_current][x_current];
                current_state.display_buffer[y_current][x_current] ^= pixel;
            }
        }
        current_state.should_draw = true;
        current_state
    }

    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn _exa1(&self, instruction: u16) -> State {
        let current_state = self.state.clone();
        let _key_index = Self::_x(instruction);
        println!("We are supposed to check if the key is pressed");
        current_state
    }

    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn _fx07(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        current_state.registers[register_index] = current_state.delay_timer;
        current_state
    }

    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn _fx15(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let timer_value = current_state.registers[register_index];
        println!("Timer value: {}", timer_value);
        current_state.delay_timer = timer_value;
        current_state
    }

    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    fn _fx18(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let register_index = Self::_x(instruction) as usize;
        let timer_value = current_state.registers[register_index];
        current_state.sound_timer = timer_value;
        current_state
    }

    /// Set I = location of sprite for digit Vx.
    fn _fx29(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let digit = Self::_x(instruction) as u16;
        // each digit is 5 bytes, so we can just multiply the digit by 5 to retrieve the index
        current_state.i = digit * 5;
        current_state
    }

    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    fn _fx33(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let value = Self::_x(instruction);
        let address_register = current_state.i as usize;
        current_state.memory[address_register] = (value / 100) % 10;
        current_state.memory[address_register + 1] = (value / 10) % 10;
        current_state.memory[address_register + 2] = value % 10;
        current_state
    }

    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn _fx65(&self, instruction: u16) -> State {
        let mut current_state = self.state.clone();
        let last_register_index = Self::_x(instruction) as usize;
        let address_register = current_state.i as usize;

        for i in 0..last_register_index + 1 {
            current_state.registers[i] = current_state.memory[address_register + i];
        }
        current_state
    }

    fn _nnn(instruction: u16) -> u16 {
        instruction & 0x0FFF
    }

    fn _n(instruction: u16) -> u8 {
        (instruction & 0x000F) as u8
    }

    fn _x(instruction: u16) -> u8 {
        ((instruction & 0x0F00) >> 8) as u8
    }

    fn _y(instruction: u16) -> u8 {
        ((instruction & 0x00F0) >> 4) as u8
    }

    fn _kk(instruction: u16) -> u8 {
        (instruction & 0x00FF) as u8
    }
}
