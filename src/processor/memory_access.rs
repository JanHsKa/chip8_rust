use crate::processor::{Memory, memory_constants};
use std::cell::RefCell;
use std::rc::Rc;

use self::memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE};

pub struct MemoryAccess {
    memory: Rc<Memory>,
}

impl MemoryAccess {
    pub fn new(memory_ref: Rc<Memory>) -> MemoryAccess {
        MemoryAccess {
            memory: Rc::clone(&memory_ref),
        }
    }

    pub fn get_graphic_array(&mut self) -> [u8; GRAPHIC_SIZE] {
        self.memory.grapphic_array.clone()
    }

    pub fn get_opcode(&mut self) -> u16 {
        self.memory.opcode
    }

    pub fn get_program_counter(&mut self) -> usize {
        self.memory.program_counter
    }
}