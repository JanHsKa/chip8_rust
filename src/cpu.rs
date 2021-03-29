use crate::constants;
use crate::keypad::Keypad;
use crate::fontset;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;

use rand::Rng;
use constants::MEMORYSIZE;
use constants::VARIABLES_COUNT;
use constants::COLUMNS;
use constants::ROWS;
use constants::STACKSIZE;
use constants::CARRY_FLAG;
use constants::MAX_PROGRAM_SIZE;
use constants::PROGRAM_START;

pub struct Cpu {
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
    keypad: Rc<RefCell<Keypad>>,
    running: bool,
}

impl Cpu {
    pub fn new(new_keypad: Rc<RefCell<Keypad>>) -> Cpu {
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
            keypad: new_keypad,
            running: true,
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
        self.opcode = (self.memory[self.program_counter] as u16) << 8 | (self.memory[self.program_counter + 1] as u16);
    }

    pub fn get_state(&mut self) -> bool {
        return self.running;
    }

    pub fn run_opcode(&mut self) {
        if self.running {
            self.set_opcode();
            //self.print_memory();
            self.decode_opcode();
        }
    }

    fn print_memory(&mut self) {
        println!("opcode: {:#04X?}", self.opcode);
        println!("program counter: {:#04X?}", self.program_counter);
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn get_graphic_array(&mut self) -> [u8; COLUMNS * ROWS] {
        return self.grapphic_array.clone();
    }

    fn print_graphic_array(&mut self) {
        for row in 0..ROWS as usize {
            for column in 0..COLUMNS {
                print!("{},", self.grapphic_array[(row * COLUMNS) + column]);
            }
            print!("\n");
        }
        print!("\n");
        print!("\n");
    }

    fn print_fontset(&mut self) {
        for i in 0..80 {
            if i % 5 == 0 {
                print!("\n");
            }
            print!("{:#02X?}, ", self.memory[i]);
        }
    }

    fn no_match(&mut self) {
        self.running = false;
        println!("Error: No matching opcode");
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

        match nibbles {
            [0x0, 0x0, 0xe, 0x0] => self.op_00e0(),
            [0x0, 0x0, 0xe, 0xe] => self.op_00ee(),
            [0x1, _, _, _] => self.op_1nnn(nnn),
            [0x2, _, _, _] => self.op_2nnn(nnn),
            [0x3, _, _, _] => self.op_3xkk(x, kk),
            [0x4, _, _, _] => self.op_4xkk(x, kk),
            [0x5, _, _, 0x0] => self.op_5xy0(x, y),
            [0x6, _, _, _] => self.op_6xkk(x, kk),
            [0x7, _, _, _] => self.op_7xkk(x, kk),
            [0x8, _, _, 0x0] => self.op_8xy0(x, y),
            [0x8, _, _, 0x1] => self.op_8xy1(x, y),
            [0x8, _, _, 0x2] => self.op_8xy2(x, y),
            [0x8, _, _, 0x3] => self.op_8xy3(x, y),
            [0x8, _, _, 0x4] => self.op_8xy4(x, y),
            [0x8, _, _, 0x5] => self.op_8xy5(x, y),
            [0x8, _, _, 0x6] => self.op_8xy6(x),
            [0x8, _, _, 0x7] => self.op_8xy7(x, y),
            [0x8, _, _, 0xE] => self.op_8xye(x),
            [0x9, _, _, 0x0] => self.op_9xy0(x, y),
            [0xA, _, _, _] => self.op_annn(nnn as u16),
            [0xB, _, _, _] => self.op_bnnn(nnn as u16),
            [0xC, _, _, _] => self.op_cxkk(x, kk),
            [0xD, _, _, _] => self.op_dxyn(x, y, n),
            [0xE, _, 0x9, 0xE] => self.op_ex9e(x),
            [0xE, _, 0xA, 0x1] => self.op_exa1(x),
            [0xF, _, 0x0, 0x7] => self.op_fx07(x),
            [0xF, _, 0x0, 0xA] => self.op_fx0a(x),
            [0xF, _, 0x1, 0x5] => self.op_fx15(x),
            [0xF, _, 0x1, 0x8] => self.op_fx18(x),
            [0xF, _, 0x1, 0xE] => self.op_fx1e(x),
            [0xF, _, 0x2, 0x9] => self.op_fx29(x),
            [0xF, _, 0x3, 0x3] => self.op_fx33(x),
            [0xF, _, 0x5, 0x5] => self.op_fx55(x),
            [0xF, _, 0x6, 0x5] => self.op_fx65(x),
            _ => self.no_match(),
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
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
        self.stack[self.stack_pointer] = 0;
    }

    //JP addr
    fn op_1nnn(&mut self, nnn: usize) {
        self.program_counter = nnn;
    }
    
    //CALL addr
    fn op_2nnn(&mut self, nnn: usize) {
        self.program_counter += 2;
        self.stack[self.stack_pointer] = self.program_counter as u16;
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
        self.variable_register[x] = self.variable_register[x].overflowing_add(kk).0;
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
        let overflow_sum = self.variable_register[x].overflowing_add(self.variable_register[y]);
        self.variable_register[x] = overflow_sum.0;
        if overflow_sum.1 {
            self.variable_register[CARRY_FLAG] = 1;
        } else {
            self.variable_register[CARRY_FLAG] = 0;
        }
        self.program_counter += 2;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.variable_register[CARRY_FLAG] = 0;

        if  self.variable_register[x] > self.variable_register[y] {
            self.variable_register[CARRY_FLAG] = 1;
        }
        self.variable_register[x] = self.variable_register[x].overflowing_sub(self.variable_register[y]).0;
        
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

        self.variable_register[y] = self.variable_register[y].overflowing_sub(self.variable_register[x]).0;
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
            y_coordinate = (self.variable_register[y] as usize + row) % ROWS;
            sprite = self.memory[self.index_register as usize + row];
            for column in 0..8 {
                x_coordinate = (self.variable_register[x] as usize + column) % COLUMNS;
                if (sprite & (0x80 >> column)) != 0 {
                    if self.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] == 1 {
                        self.variable_register[CARRY_FLAG] = 1;
                    }
                    self.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] ^= 1;
                }
            }
        }
        self.program_counter += 2;
    }

    //SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        if (*self.keypad.borrow_mut()).get_key(self.variable_register[x]) == 1 {
            self.program_counter += 2;
        }
        self.program_counter += 2;
    }

     //SKNP Vx
     fn op_exa1(&mut self, x: usize) {
        if (*self.keypad.borrow_mut()).get_key(self.variable_register[x]) == 0 {
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
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if (*keypad_borrow).is_any_key_pressed() {
            self.variable_register[x] = (*keypad_borrow).get_pressed_key();
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
        println!("x: {}", x);
        println!("getting char at: {}", self.variable_register[x]);
        self.index_register = (self.variable_register[x] as u16) * 0x5;
        println!("index : {}", self.index_register);
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




