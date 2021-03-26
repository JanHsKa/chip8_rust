use crate::constants;

use constants::KEY_COUNT;

pub struct Keypad {
    keys: [u8; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad{
            keys: [0; 16],
        }
    }

    pub fn get_key(&mut self, key: u8) -> u8 {
        return self.keys[key as usize];
    }
    
    pub fn get_pressed_key(&mut self) -> u8 {
        for i in 0..KEY_COUNT {
            if self.keys[i] == 1 {
                return i as u8;
            }
        }
        return 0;
    }

    pub fn is_any_key_pressed(&mut self) -> bool {
        return true;
    }
}