use crate::processor::{Memory, memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE}};

use std::{cell::RefCell, rc::Rc, option::Option};

pub struct MemoryAccess {
    memory: Rc<RefCell<Memory>>,
}

impl MemoryAccess {
    pub fn new(memory_ref: Rc<RefCell<Memory>>) -> MemoryAccess {
        MemoryAccess {
            memory: memory_ref,
        }
    }

    pub fn get_graphic_array(&mut self) -> [u8; GRAPHIC_SIZE] {
        self.memory.borrow().grapphic_array.clone()
    }

    pub fn get_opcode(&mut self) -> u16 {
        self.memory.borrow().opcode
    }

    pub fn get_program_counter(&mut self) -> usize {
        self.memory.borrow().program_counter
    }

    pub fn get_complete_memory(&mut self) -> Vec<u8> {
        let mut memory_content = vec![0; MEMORYSIZE];
        memory_content.copy_from_slice(&self.memory.borrow().memory);
        
        memory_content
    }

    pub fn get_stack(&mut self) -> Vec<u16> {
        let mut stack = vec![0; STACKSIZE];
        stack.copy_from_slice(&self.memory.borrow().stack);
        
        stack
    }

    pub fn get_stack_pointer(&mut self) -> usize {
        self.memory.borrow().stack_pointer
    }

    pub fn get_variable_register(&mut self) -> Vec<u8> {
        let mut variable_register = vec![0; VARIABLES_COUNT];
        variable_register.copy_from_slice(&self.memory.borrow().variable_register);
        
        variable_register
    }

    pub fn get_index_register(&mut self) -> u16 {
        self.memory.borrow().index_register
    }
    
    pub fn get_delay_timer(&mut self) -> u8 {
        self.memory.borrow().delay_timer
    }

    pub fn get_sound_timer(&mut self) -> u8 {
        self.memory.borrow().sound_timer
    }
}