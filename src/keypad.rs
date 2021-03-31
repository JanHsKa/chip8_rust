use crate::processor::memory_constants;
use crate::sdl2::keyboard::Keycode;
use std::collections;

use memory_constants::KEY_COUNT;

pub struct Keypad {
    keys: [u8; 16],
    keymap: collections::HashMap<Keycode, usize>,
}

impl Keypad {
    pub fn new() -> Keypad {
        let mut new_keymap = collections::HashMap::new();
        new_keymap.insert(Keycode::Num1, 0x1);
        new_keymap.insert(Keycode::Num2, 0x2);
        new_keymap.insert(Keycode::Num3, 0x3);
        new_keymap.insert(Keycode::Num4, 0xC);
        new_keymap.insert(Keycode::Q, 0x4);
        new_keymap.insert(Keycode::W, 0x5);
        new_keymap.insert(Keycode::E, 0x6);
        new_keymap.insert(Keycode::R, 0xD);
        new_keymap.insert(Keycode::A, 0x7);
        new_keymap.insert(Keycode::S, 0x8);
        new_keymap.insert(Keycode::D, 0x9);
        new_keymap.insert(Keycode::F, 0xE);
        new_keymap.insert(Keycode::Y, 0xA);
        new_keymap.insert(Keycode::X, 0x0);
        new_keymap.insert(Keycode::C, 0xB);
        new_keymap.insert(Keycode::V, 0xF);

        Keypad{
            keys: [0; KEY_COUNT],
            keymap: new_keymap,
        }
    }

    pub fn press_key(&mut self, key: Keycode, value: u8) {
        if self.keymap.contains_key(&key) {
            self.keys[self.keymap[&key]] = value;
        }
    }

    pub fn get_key(&mut self, key: u8) -> u8 {
        return self.keys[key as usize];
    }
    
    pub fn get_pressed_key(&mut self) -> Option<u8> {
        for i in 0..KEY_COUNT {
            if self.keys[i] != 0 {
                return Some(i as u8);
            }
        }
        return None;
    }

    pub fn reset_key(&mut self, key: u8) {
        if key < KEY_COUNT as u8{
            self.keys[key as usize] = 0;
        }
    }

    pub fn is_any_key_pressed(&mut self) -> bool {
        for i in 0..KEY_COUNT {
            if self.keys[i] != 0 {
                return true;
            }
        }
        return false;
    }

    pub fn print_keys(&mut self) {
        for i in self.keys.iter() {
            print!("{}, ", *i);
        }
        println!("");
    }
}