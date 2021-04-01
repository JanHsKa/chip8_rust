use crate::processor::memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE};

pub struct Memory {
    pub memory:[u8; MEMORYSIZE],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub grapphic_array: [u8; COLUMNS * ROWS],
    pub variable_register: [u8; VARIABLES_COUNT], 
    pub stack_pointer: usize,
    pub program_counter: usize,
    pub stack: [u16; STACKSIZE],
    pub opcode: u16,
    pub index_register: u16,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORYSIZE],
            delay_timer: 0,
            sound_timer: 0,
            grapphic_array: [0; COLUMNS * ROWS],
            variable_register: [0; STACKSIZE], 
            stack_pointer: 0,
            program_counter: PROGRAM_START,
            stack: [0; STACKSIZE],
            opcode: 0,
            index_register: 0,
        }
    }
}