pub const MEMORYSIZE: usize = 4096;
pub const STACKSIZE: usize = 16;
pub const VARIABLES_COUNT: usize = 16;
pub const COLUMNS: usize = 64;
pub const ROWS: usize = 32;
pub const KEY_COUNT: usize = 16;
pub const MAX_PROGRAM_SIZE: usize = 3584;
pub const PROGRAM_START: usize = 0x200;

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