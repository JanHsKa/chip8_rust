use crate::defines::{memory_constants::KEY_COUNT, KeyPress, KeyReset};
use crate::sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct Keypad {
    keys: [u8; 16],
    keymap: HashMap<Keycode, usize>,
    key_reset: KeyReset,
}

impl Default for Keypad {
    fn default() -> Self {
        Keypad::new()
    }
}

impl Keypad {
    pub fn new() -> Keypad {
        let new_keymap: HashMap<Keycode, usize> = vec![
            (Keycode::Num1, 0x1),
            (Keycode::Num2, 0x2),
            (Keycode::Num3, 0x3),
            (Keycode::Num4, 0xC),
            (Keycode::Q, 0x4),
            (Keycode::W, 0x5),
            (Keycode::E, 0x6),
            (Keycode::R, 0xD),
            (Keycode::A, 0x7),
            (Keycode::S, 0x8),
            (Keycode::D, 0x9),
            (Keycode::F, 0xE),
            (Keycode::Y, 0xA),
            (Keycode::X, 0x0),
            (Keycode::C, 0xB),
            (Keycode::V, 0xF),
        ]
        .into_iter()
        .collect();

        Keypad {
            keys: [0; KEY_COUNT],
            keymap: new_keymap,
            key_reset: KeyReset::Static,
        }
    }

    pub fn toggle_key_reset(&mut self) {
        let key_reset = self.key_reset;

        match key_reset {
            KeyReset::Smooth => self.key_reset = KeyReset::Static,
            KeyReset::Static => self.key_reset = KeyReset::Smooth,
        }
    }

    pub fn press_key(&mut self, key: Keycode, value: KeyPress) {
        if self.keymap.contains_key(&key) {
            self.keys[self.keymap[&key]] = value as u8;
        }
    }

    pub fn get_key(&mut self, key: u8) -> u8 {
        self.keys[key as usize]
    }

    pub fn get_pressed_key(&mut self) -> Option<u8> {
        for i in 0..KEY_COUNT {
            if self.keys[i] != 0 {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn reset_key(&mut self, key: u8) {
        if self.key_reset == KeyReset::Static {
            if key < KEY_COUNT as u8 {
                self.keys[key as usize] = 0;
            }
        }
    }

    pub fn is_any_key_pressed(&mut self) -> bool {
        for i in 0..KEY_COUNT {
            if self.keys[i] != 0 {
                return true;
            }
        }
        false
    }

    pub fn print_keys(&mut self) {
        for i in self.keys.iter() {
            print!("{}, ", *i);
        }
        println!();
    }
}
