use crate::defines::memory_constants::{
    FLAG_REGISTER_SIZE, GRAPHIC_SIZE, MEMORYSIZE, PROGRAM_START, STACKSIZE, VARIABLES_COUNT,
};

#[derive(Copy, Clone, PartialEq)]
pub enum Resolution {
    Low = 1,
    High = 2,
}

pub struct Memory {
    pub memory: [u8; MEMORYSIZE],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphic_array: Vec<u8>,
    pub variable_register: [u8; VARIABLES_COUNT],
    pub stack_pointer: usize,
    pub program_counter: usize,
    pub stack: [u16; STACKSIZE],
    pub opcode: u16,
    pub index_register: u16,
    pub flag_register: [u8; FLAG_REGISTER_SIZE],
    pub resolution: Resolution,
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORYSIZE],
            delay_timer: 0,
            sound_timer: 0,
            graphic_array: vec![0; GRAPHIC_SIZE],

            variable_register: [0; STACKSIZE],
            stack_pointer: 0,
            program_counter: PROGRAM_START,
            stack: [0; STACKSIZE],
            opcode: 0,
            index_register: 0,
            flag_register: [0; FLAG_REGISTER_SIZE],
            resolution: Resolution::Low,
        }
    }

    pub fn reset(&mut self) {
        *self = Memory::new();
    }
}
