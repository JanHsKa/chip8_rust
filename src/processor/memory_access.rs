use crate::processor::{Memory, Resolution, memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, MAX_PROGRAM_SIZE,  
    PROGRAM_STEP, GRAPHIC_SIZE}};

use std::{result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}}};

pub struct MemoryAccess {
    memory: Arc<Mutex<Memory>>,
}

impl MemoryAccess {
    pub fn new(memory_ref: Arc<Mutex<Memory>>) -> MemoryAccess {
        MemoryAccess {
            memory: memory_ref,
        }
    }

    pub fn get_graphic_array(&mut self) -> Vec<u8> {
        self.memory.lock().unwrap().grapphic_array.clone()
    }

    pub fn get_opcode(&mut self) -> u16 {
        self.memory.lock().unwrap().opcode
    }

    pub fn get_program_counter(&mut self) -> usize {
        self.memory.lock().unwrap().program_counter
    }

    pub fn get_complete_memory(&mut self) -> Vec<u8> {
        let mut memory_content = vec![0; MEMORYSIZE];
        memory_content.copy_from_slice(&self.memory.lock().unwrap().memory);
        
        memory_content
    }

    pub fn get_stack(&mut self) -> Vec<u16> {
        let mut stack = vec![0; STACKSIZE];
        stack.copy_from_slice(&self.memory.lock().unwrap().stack);
        
        stack
    }

    pub fn get_stack_pointer(&mut self) -> usize {
        self.memory.lock().unwrap().stack_pointer
    }

    pub fn get_variable_register(&mut self) -> Vec<u8> {
        let mut variable_register = vec![0; VARIABLES_COUNT];
        variable_register.copy_from_slice(&self.memory.lock().unwrap().variable_register);
        
        variable_register
    }

    pub fn get_index_register(&mut self) -> u16 {
        self.memory.lock().unwrap().index_register
    }
    
    pub fn get_delay_timer(&mut self) -> u8 {
        self.memory.lock().unwrap().delay_timer
    }

    pub fn get_sound_timer(&mut self) -> u8 {
        self.memory.lock().unwrap().sound_timer
    }

    pub fn get_resolution(&mut self) -> Resolution {
        self.memory.lock().unwrap().resolution
    }
}