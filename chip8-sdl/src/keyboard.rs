use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use chip8::keyboard::KeyState;
use chip8::keyboard::NUM_KEYS;

#[derive(Clone, Copy, Debug)]
pub enum PollError {
    Quit,
}

pub trait Keyboard {
    fn poll(&mut self) -> Result<[KeyState; NUM_KEYS], PollError>;
}

pub struct SdlKeyboard {
    event_pump: sdl2::EventPump,
}

impl SdlKeyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        SdlKeyboard {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }
}

impl Keyboard for SdlKeyboard {
    fn poll(&mut self) -> Result<[KeyState; NUM_KEYS], PollError> {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(PollError::Quit);
            }
        }

        let mut keys = [KeyState::Up; NUM_KEYS];

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .filter_map(translate_key)
            .for_each(|key| keys[key] = KeyState::Down);

        Ok(keys)
    }
}

fn translate_key(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::X => Some(0x0),
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::Z => Some(0xa),
        Keycode::C => Some(0xb),
        Keycode::Num4 => Some(0xc),
        Keycode::R => Some(0xd),
        Keycode::F => Some(0xe),
        Keycode::V => Some(0xf),
        _ => None,
    }
}