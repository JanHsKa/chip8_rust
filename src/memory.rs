use crate::constants;

use constants::MEMORYSIZE;
use constants::VARIABLES_COUNT;
use constants::COLUMNS;
use constants::ROWS;
use constants::STACKSIZE;
use constants::CARRY_FLAG;
use constants::MAX_PROGRAM_SIZE;
use constants::PROGRAM_START;

pub struct Memory {
    memory:[u8; MEMORYSIZE],
    delay_timer: u8,
    sound_timer: u8,
    grapphic_array: [u8; COLUMNS * ROWS],
    variable_register: [u8; VARIABLES_COUNT], 
    stack_pointer: usize,
    program_counter: usize,
    stack: [u16; STACKSIZE],
    opcode: u16,
    index_register: u16,
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