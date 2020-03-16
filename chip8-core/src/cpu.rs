use rand::prelude::*;

use crate::keyboard::{KeyState, Keys};
use crate::state::{State, DISPLAY_HEIGHT, DISPLAY_WIDTH};

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

    pub fn iteration(&mut self, keys: &Keys) -> Option<State> {
        if self.state.waiting_for_key {
            if let Some(key) = keys.iter().position(|&ks| ks == KeyState::Down) {
                self.state.registers[self.state.key_register_index] = key as u8;
                self.state.waiting_for_key = false;
            }
        } else {
            let instruction = self.state.instruction();
            self.state.next_instruction();
            self.update_timers();
            self.state.should_draw = false;
            self.state.play_audio = self.state.sound_timer > 0;
            self.parse_instruction(instruction, keys);
        }
        Some(self.state.clone())
    }

    fn update_timers(&mut self) {
        if self.state.delay_timer > 0 {
            self.state.delay_timer -= 1;
        }
        if self.state.sound_timer > 0 {
            self.state.sound_timer -= 1;
        }
    }

    fn parse_instruction(&mut self, instruction: u16, keys: &Keys) {
        match instruction & 0xF000 {
            0x0000 => match instruction & 0x00FF {
                0x00E0 => self._00e0(),
                0x00EE => self._00ee(),
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
                0x0006 => self._8xy6(instruction),
                0x0007 => self._8xy7(instruction),
                0x000E => self._8xye(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            0x9000 => self._9xy0(instruction),
            0xA000 => self._annn(instruction),
            0xB000 => self._bnnn(instruction),
            0xC000 => self._cxkk(instruction),
            0xD000 => self._dxyn(instruction),
            0xE000 => match instruction & 0x00FF {
                0x009E => self._ex9e(instruction, keys),
                0x00A1 => self._exa1(instruction, keys),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            0xF000 => match instruction & 0x00FF {
                0x0007 => self._fx07(instruction),
                0x000A => self._fx0a(instruction),
                0x0015 => self._fx15(instruction),
                0x0018 => self._fx18(instruction),
                0x001E => self._fx1e(instruction),
                0x0033 => self._fx33(instruction),
                0x0055 => self._fx55(instruction),
                0x0065 => self._fx65(instruction),
                0x0029 => self._fx29(instruction),
                _ => panic!("Unknown opcode: {:X}", instruction),
            },
            _ => panic!("Unknown opcode {:X}", instruction),
        };
    }

    /// Clear the display.
    fn _00e0(&mut self) {
        self.state
            .display_buffer
            .iter_mut()
            .for_each(|value| value.iter_mut().for_each(|px| *px = 0));
    }

    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn _00ee(&mut self) {
        let return_address = self.state.stack.pop();
        self.state.pc = return_address as usize;
    }

    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn _1nnn(&mut self, instruction: u16) {
        let address = Self::_nnn(instruction);
        self.state.pc = address as usize;
    }

    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
    /// The PC is then set to nnn.
    fn _2nnn(&mut self, instruction: u16) {
        let call_address = Self::_nnn(instruction);
        self.state.stack.push(self.state.pc as u16);
        self.state.pc = call_address as usize;
    }

    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn _3xkk(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let vx: u8 = self.state.registers[register_index];
        if vx == value {
            self.state.next_instruction();
        }
    }

    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn _4xkk(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let vx: u8 = self.state.registers[x];
        if vx != value {
            self.state.next_instruction();
        }
    }

    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn _5xy0(&mut self, instruction: u16) {
        let register_x_index = Self::_x(instruction) as usize;
        let register_y_index = Self::_y(instruction) as usize;
        let vx: u8 = self.state.registers[register_x_index];
        let vy: u8 = self.state.registers[register_y_index];

        if vx == vy {
            self.state.next_instruction();
        }
    }

    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn _6xkk(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let register_value = Self::_kk(instruction);
        self.state.registers[register_index] = register_value;
    }

    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn _7xkk(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let vx = self.state.registers[register_index] as u8;
        let add_result = vx.wrapping_add(value);
        self.state.registers[register_index] = add_result;
    }

    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn _8xy0(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        self.state.registers[x] = self.state.registers[y];
    }

    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn _8xy1(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        self.state.registers[x] = vx | vy;
    }

    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn _8xy2(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        self.state.registers[x] = vx & vy;
    }

    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same,
    /// then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn _8xy3(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        self.state.registers[x] = vx ^ vy;
    }

    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn _8xy4(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx = self.state.registers[x] as u16;
        let vy = self.state.registers[y] as u16;
        let result = vx + vy;
        self.state.registers[0x0F] = (result > std::u8::MAX as u16).into();
        self.state.registers[x] = (result & 0x00FF) as u8;
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    fn _8xy5(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_y(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        self.state.registers[0x0F] = (vx > vy).into();
        self.state.registers[x] = vx.wrapping_sub(vy);
    }

    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    fn _8xy6(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        self.state.registers[0x0F] = vx & 1;
        self.state.registers[x] >>= 1;
    }

    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn _8xy7(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_x(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        self.state.registers[0x0F] = (vy > vx).into();
        self.state.registers[x] = vy.wrapping_sub(vx);
    }

    // Set Vx = Vx SHL 1.
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn _8xye(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        self.state.registers[0x0F] = (vx & 0b10000000) >> 7;
        self.state.registers[x] <<= 1;
    }

    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn _9xy0(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        let y = Self::_x(instruction) as usize;
        let vx: u8 = self.state.registers[x];
        let vy: u8 = self.state.registers[y];
        if vx != vy {
            self.state.next_instruction();
        }
    }

    /// Set I = nnn.
    /// The value of register I is set to nnn.
    fn _annn(&mut self, instruction: u16) {
        let address = Self::_nnn(instruction);
        self.state.set_address_register(address);
    }

    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    fn _bnnn(&mut self, instruction: u16) {
        let address = Self::_nnn(instruction) as usize;
        let v0 = self.state.registers[0] as usize;
        self.state.pc = address + v0;
    }

    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx.
    fn _cxkk(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let value = Self::_kk(instruction);
        let random_value: u8 = random();
        self.state.registers[register_index] = random_value & value;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    fn _dxyn(&mut self, _instruction: u16) {
        let register_x = Self::_x(_instruction) as usize;
        let register_y = Self::_y(_instruction) as usize;
        let x_start = self.state.registers[register_x] as usize;
        let y_start = self.state.registers[register_y] as usize;
        let n = Self::_n(_instruction) as usize;
        let width = 8; // width of the sprite
        let mut collision = false;

        for y in 0..n {
            let address_register = self.state.i as usize;
            let current_pixel_index = address_register + y;
            for x in 0..width {
                let x_current = (x_start + x) % DISPLAY_WIDTH;
                let y_current = (y_start + y) % DISPLAY_HEIGHT;
                let pixel = self.state.ram.get(current_pixel_index);

                if pixel & (0x80 >> x) != 0 {
                    if self.state.display_buffer[y_current][x_current] == 1 {
                        collision = true;
                    }
                    self.state.display_buffer[y_current][x_current] ^= 1;
                }
            }
        }
        self.state.registers[0x0F] = collision.into();
        self.state.should_draw = true;
    }

    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently
    /// in the down position, PC is increased by 2.
    fn _ex9e(&mut self, instruction: u16, keys: &Keys) {
        let register_index = Self::_x(instruction) as usize;
        let key_index = self.state.registers[register_index] as usize;
        if keys[key_index] == KeyState::Down {
            self.state.next_instruction();
        }
    }

    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn _exa1(&mut self, instruction: u16, keys: &Keys) {
        let register_index = Self::_x(instruction) as usize;
        let key_index = self.state.registers[register_index] as usize;

        if keys[key_index] == KeyState::Up {
            self.state.next_instruction();
        }
    }

    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn _fx07(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        self.state.registers[register_index] = self.state.delay_timer;
    }

    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn _fx0a(&mut self, instruction: u16) {
        let x = Self::_x(instruction) as usize;
        self.state.key_register_index = x;
        self.state.waiting_for_key = true;
    }

    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn _fx15(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let timer_value: u8 = self.state.registers[register_index];
        self.state.delay_timer = timer_value;
    }

    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    fn _fx18(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let timer_value: u8 = self.state.registers[register_index];
        self.state.sound_timer = timer_value;
    }

    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn _fx1e(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let vx = self.state.registers[register_index] as u16;
        let result = vx as u32 + self.state.i as u32;
        self.state.registers[0x0F] = (result > std::u8::MAX as u32).into();
        self.state.i = self.state.i.wrapping_add(vx);
    }

    /// Set I = location of sprite for digit Vx.
    fn _fx29(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let index = self.state.registers[register_index] as u16;
        // each digit is 5 bytes, so we can just multiply the digit by 5 to retrieve the index
        self.state.i = index * 5;
    }

    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    fn _fx33(&mut self, instruction: u16) {
        let register_index = Self::_x(instruction) as usize;
        let vx: u8 = self.state.registers[register_index];
        let address_register = self.state.i as usize;
        self.state.ram.set(address_register, (vx / 100) % 10);
        self.state.ram.set(address_register + 1, (vx / 10) % 10);
        self.state.ram.set(address_register + 2, vx % 10);
    }

    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn _fx55(&mut self, instruction: u16) {
        let last_register_index = Self::_x(instruction) as usize;

        for index in 0..last_register_index + 1 {
            let value: u8 = self.state.registers[index];
            self.state.ram.set(self.state.i as usize + index, value);
        }
    }

    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn _fx65(&mut self, instruction: u16) {
        let last_register_index = Self::_x(instruction) as usize;
        let address_register = self.state.i as usize;

        for i in 0..last_register_index + 1 {
            self.state.registers[i] = self.state.ram.get(address_register + i);
        }
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
