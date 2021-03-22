use crate::constants;
use constants::MEMORYSIZE;
use constants::COLUMNS;
use constants::ROWS;
use constants::STACKSIZE;

pub struct Cpu{
    memory:[u8; MEMORYSIZE],
    delay_timer: u8,
    sound_timer: u8,
    grapphic_array: [u8; COLUMNS * ROWS],
    variable_register: [u8; STACKSIZE], 
    stack_pointer: usize,
    program_counter: usize,
    stack: [u16; STACKSIZE],
    opcode: u16,

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            memory: [0; MEMORYSIZE],
            delay_timer: 0,
            sound_timer: 0,
            grapphic_array: [0; COLUMNS * ROWS],
            variable_register: [0; STACKSIZE], 
            stack_pointer: 0,
            program_counter: 0x200,
            stack: [0; STACKSIZE],
            opcode: 0,
        }
    }

    fn set_opcode(&mut self) {
        self.opcode = (self.memory[self.program_counter] << 4 | self.memory[self.program_counter + 1]) as u16;
    }

    pub fn run_opcode(&mut self) {
        self.decode_opcode();
    }

    fn decode_opcode(&mut self) {
        let nibbles = [
            (self.opcode & 0xF000) >> 12,
            (self.opcode & 0x0F00) >> 8,
            (self.opcode & 0x00F0) >> 4,
            self.opcode & 0x000F
        ];
    }

    fn op_00e0(&mut self) {

    }

    fn op_00ee(&mut self, nibbles: [u16; 4]) {
        self.opcode += 1;
        self.stack[self.stack_pointer] = self.opcode;
        self.stack_pointer += 1;
        self.opcode = (nibbles[1] << 8) | (nibbles[2] << 4) | nibbles[3];
        
    }
}




