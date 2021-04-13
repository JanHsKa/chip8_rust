use crate::defines::memory_constants::{
    COLUMNS, FLAG_REGISTER_SIZE, GRAPHIC_SIZE, MAX_PROGRAM_SIZE, MEMORYSIZE, PROGRAM_START,
    PROGRAM_STEP, ROWS, STACKSIZE, VARIABLES_COUNT,
};

use crate::model::{Memory, Resolution};
use std::sync::{Arc, Mutex};

pub struct MemoryAccess {
    memory: Arc<Mutex<Memory>>,
}

impl MemoryAccess {
    pub fn new(memory_ref: Arc<Mutex<Memory>>) -> MemoryAccess {
        MemoryAccess { memory: memory_ref }
    }

    pub fn get_graphic_array(&mut self) -> Vec<u8> {
        self.memory.lock().unwrap().graphic_array.clone()
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

    pub fn get_flag_register(&mut self) -> Vec<u8> {
        let mut flag_register = vec![0; FLAG_REGISTER_SIZE];
        flag_register.copy_from_slice(&self.memory.lock().unwrap().flag_register);

        flag_register
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

    pub fn get_code_snippet(&mut self, count: usize, offset: usize) -> Option<Vec<u16>> {
        if offset + count * 2 + 1 >= MAX_PROGRAM_SIZE {
            return None;
        }

        let mut code_lines: Vec<u16> = vec![0; count];
        let memory_content = &self.memory.lock().unwrap().memory[PROGRAM_START..];
        for (i, iter) in code_lines.iter_mut().enumerate() {
            *iter = (memory_content[offset + 2 * i] as u16) << 8
                | memory_content[offset + 2 * i + 1] as u16;
        }

        Some(code_lines)
    }

    pub fn get_opcode_at(&mut self, line: usize) -> Option<u16> {
        if line + 1 >= MAX_PROGRAM_SIZE {
            return None;
        }
        let memory_line = line + PROGRAM_START;
        let memory = self.memory.lock().unwrap();
        let opcode: u16 =
            (memory.memory[memory_line] as u16) << 8 | memory.memory[memory_line + 1] as u16;

        Some(opcode)
    }
}
