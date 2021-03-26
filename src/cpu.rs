use crate::constants;
use crate::keypad::Keypad;
use crate::fontset;

use rand::Rng;
use constants::MEMORYSIZE;
use constants::VARIABLES_COUNT;
use constants::COLUMNS;
use constants::ROWS;
use constants::STACKSIZE;
use constants::CARRY_FLAG;
use constants::MAX_PROGRAM_SIZE;
use constants::PROGRAM_START;

pub struct Cpu{
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
    keypad: Keypad,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu{
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
            keypad: Keypad::new(),
        };

        for i in 0..fontset::FONTSET.len() {
            cpu.memory[i] = fontset::FONTSET[i];
        }

        return cpu;
    }

    pub fn load_program_code(&mut self, code: [u8; MAX_PROGRAM_SIZE]) {
        for i in 0..MAX_PROGRAM_SIZE {
            self.memory[i + PROGRAM_START] = code[i];
        }
    }

    fn set_opcode(&mut self) {
        self.opcode = (self.memory[self.program_counter] << 4 | self.memory[self.program_counter + 1]) as u16;
    }

    pub fn run_opcode(&mut self) {
        self.set_opcode();
        self.decode_opcode();
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn decode_opcode(&mut self) {
        let nibbles = [
            (self.opcode & 0xF000) >> 12,
            (self.opcode & 0x0F00) >> 8,
            (self.opcode & 0x00F0) >> 4,
            self.opcode & 0x000F
        ];

        let x = nibbles[1] as usize;
        let y = nibbles[2] as usize;
        let nnn = (nibbles[1] << 8 | nibbles[2] << 4 | nibbles[3]) as usize;
        let kk = (nibbles[2] << 4 | nibbles[3]) as u8;
        let n = nibbles[3] as usize;

        match self.opcode {
            0x00e0 => self.op_00e0(),
            0x00ee => self.op_00ee(),
            0x1___ => self.op_1nnn(nnn),
            0x2___ => self.op_2nnn(nnn),
            0x3___ => self.op_3xkk(x, kk),
            0x4___ => self.op_4xkk(x, kk),
            0x5__0 => self.op_5xy0(x, y),
            0x6___ => self.op_6xkk(x, kk),
            0x7___ => self.op_7xkk(x, kk),
            0x8__0 => self.op_8xy0(x, y),
            0x8__1 => self.op_8xy1(x, y),
            0x8__2 => self.op_8xy2(x, y),
            0x8__3 => self.op_8xy3(x, y),
            0x8__4 => self.op_8xy4(x, y),
            0x8__5 => self.op_8xy5(x, y),
            0x8__6 => self.op_8xy6(x),
            0x8__7 => self.op_8xy7(x, y),
            0x8__E => self.op_8xye(x),
            0x9__0 => self.op_9xy0(x, y),
            0xA___ => self.op_annn(nnn as u16),
            0xB___ => self.op_bnnn(nnn as u16),
            0xC___ => self.op_cxkk(x, kk),
            0xD___ => self.op_dxyn(x, y, n),
            0xE_9E => self.op_ex9e(x),
            0xE_A1 => self.op_exa1(x),
            0xF_07 => self.op_fx07(x),
            0xF_0A => self.op_fx0a(x),
            0xF_15 => self.op_fx15(x),
            0xF_18 => self.op_fx18(x),
            0xF_1E => self.op_fx1e(x),
            0xF_29 => self.op_fx29(x),
            0xF_33 => self.op_fx33(x),
            0xF_55 => self.op_fx55(x),
            0xF_65 => self.op_fx65(x),
            _ => println!("Error: No matching opcode"),
        }
    }

    //CLS
    fn op_00e0(&mut self) {
        for i in 0..self.grapphic_array.len() {
            self.grapphic_array[i] = 0;
        }
        self.program_counter += 2;
    }

    //RET from subroutine
    fn op_00ee(&mut self) {
        self.program_counter = self.stack[self.stack_pointer].into();
        self.stack_pointer -= 1;
    }

    //JP addr
    fn op_1nnn(&mut self, nnn: usize) {
        self.program_counter = nnn;
    }
    
    //CALL addr
    fn op_2nnn(&mut self, nnn: usize) {
        self.program_counter += 2;
        self.stack[self.stack_pointer] = self.opcode;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.variable_register[x] == kk {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.variable_register[x] != kk {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.variable_register[x] == self.variable_register[y] {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

    //LD Vx, byte
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.variable_register[x] = kk;
        self.program_counter += 2;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.variable_register[x] += kk;
        self.program_counter += 2;
            }

    //LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.variable_register[x] = self.variable_register[y];
        self.program_counter += 2;
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.variable_register[x] |= self.variable_register[y];
        self.program_counter += 2;
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.variable_register[x] &= self.variable_register[y];
        self.program_counter += 2;
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.variable_register[x] ^= self.variable_register[y];
        self.program_counter += 2;
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[x] as u16 + self.variable_register[y] as u16 > 0xFF {
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[x] += self.variable_register[y];
        self.program_counter += 2;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[x] > self.variable_register[y] {
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[x] -= self.variable_register[y];
        self.program_counter += 2;
    }

    //SHR Vx
    fn op_8xy6(&mut self, x: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[x] & 0x1 == 1{
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[x] >>= 1;
        self.program_counter += 2;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[y] > self.variable_register[x] {
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[y] -= self.variable_register[x];
        self.program_counter += 2;
    }

    //SHL Vx
    fn op_8xye(&mut self, x: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[x] & 0x80 == 0x80 {
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[x] <<= 1;
        self.program_counter += 2;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.variable_register[x] != self.variable_register[y] {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

    //LD I, addr
    fn op_annn(&mut self, nnn: u16) {
        self.index_register = nnn;
        self.program_counter += 2;
    }

    //JP V0, addr
    fn op_bnnn(&mut self, nnn: u16) {
        self.program_counter = (nnn + self.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.variable_register[x] = rng.gen_range(0..0xFF + 1) as u8 & kk;
        self.program_counter += 2;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let mut x_coordinate : usize;
        let mut y_coordinate : usize;
        let mut sprite : u8; 

        self.variable_register[CARRY_FLAG] = 0;

        for row in 0..n as usize {
            y_coordinate = (y + row) % ROWS;
            sprite = self.memory[self.index_register as usize + row];
            for column in 0..8 {
                x_coordinate = (x + column) % COLUMNS;
                if (sprite & column as u8) == 1 {
                    self.grapphic_array[y_coordinate * x_coordinate] ^= 1;
                    
                    if self.grapphic_array[y_coordinate * x_coordinate] == 1 {
                        self.variable_register[CARRY_FLAG] = 1;
                    }
                }
            }
        }
        self.program_counter += 2;
    }

    //SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        if self.keypad.get_key(self.variable_register[x]) == 1 {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

     //SKNP Vx
     fn op_exa1(&mut self, x: usize) {
        if self.keypad.get_key(self.variable_register[x]) == 0 {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

    //LD Vx, DT
    fn op_fx07(&mut self, x: usize) {
        self.variable_register[x] = self.delay_timer;
        self.program_counter += 2;
    }

    //LD Vx, K
    fn op_fx0a(&mut self, x: usize) {
        if self.keypad.is_any_key_pressed() {
            self.variable_register[x] = self.keypad.get_pressed_key();
            self.program_counter += 2;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.variable_register[x];
        self.program_counter += 2;
    }

    //LD ST, Vx
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.variable_register[x];
        self.program_counter += 2;
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self, x: usize) {
        self.index_register += self.variable_register[x] as u16;
        self.program_counter += 2;
    }

    //LD F, Vx
    fn op_fx29(&mut self, x: usize) {
        self.index_register = self.variable_register[x] as u16 * 0x5;
        self.program_counter += 2;
    }

    //LD B, Vx
    fn op_fx33(&mut self, x: usize) {
        if self.index_register as usize + 3 < MEMORYSIZE {
            self.memory[self.index_register as usize] = self.variable_register[x] / 100;
            self.memory[self.index_register as usize] = (self.variable_register[x] / 10) % 10;
            self.memory[self.index_register as usize] = self.variable_register[x] % 10;
        }
        self.program_counter += 2;
    }

    //LD [I], Vx
    fn op_fx55(&mut self, x: usize) {
        if self.index_register as usize + x < MEMORYSIZE {
            for i in 0..x {
                self.memory[self.index_register as usize + i] = self.variable_register[i];
            }
        }
        self.program_counter += 2;
    }    

    //LD Vx, [I]
    fn op_fx65(&mut self, x: usize) {
        if self.index_register as usize + x < MEMORYSIZE {
            for i in 0..x {
                self.variable_register[i] = self.memory[self.index_register as usize + i];
            }
        }
        self.program_counter += 2;
    }   
}




