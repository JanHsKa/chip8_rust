use crate::keypad::Keypad;
use crate::processor::{memory_constants, fontset, memory};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use self::memory::Memory;

use rand::Rng;

use self::memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE};


pub struct Cpu {
    data: Memory,
    keypad: Rc<RefCell<Keypad>>,
    running: bool,
    x: usize,
    y: usize,
    nnn: u16,
    kk: u8,
    n: usize,
}

impl Cpu {
    pub fn new(new_keypad: Rc<RefCell<Keypad>>, new_data: Memory) -> Cpu {
        let mut cpu = Cpu{
            data: new_data,
            keypad: new_keypad,
            running: true,
            x: 0,
            y: 0,
            nnn: 0,
            kk: 0,
            n: 0,
        };

        cpu.data.memory[..fontset::FONTSET.len()].copy_from_slice(&fontset::FONTSET[..]);
        
        cpu
    }

    pub fn load_program_code(&mut self, code: [u8; MAX_PROGRAM_SIZE]) {
        for i in 0..MAX_PROGRAM_SIZE {
            self.data.memory[i + PROGRAM_START] = code[i];
        }
    }

    fn set_opcode(&mut self) {
        self.data.opcode = (self.data.memory[self.data.program_counter] as u16) << 8 
            | (self.data.memory[self.data.program_counter + 1] as u16);
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
        println!("opcode: {:#04X?}", self.data.opcode);
        println!("program counter: {:#04X?}", self.data.program_counter);
    }

    pub fn tick_timer(&mut self) {
        if self.running {
            self.data.delay_timer = self.data.delay_timer.saturating_sub(1);
            
            if self.data.sound_timer > 0 {
                self.data.sound_timer -= 1;
            }
        }
    }

    pub fn get_graphic_array(&mut self) -> [u8; GRAPHIC_SIZE] {
        self.data.grapphic_array.clone()
    }
    pub fn play_sound(&mut self) -> bool {
        self.data.sound_timer > 0
    }

    fn print_graphic_array(&mut self) {
        for row in 0..ROWS as usize {
            for column in 0..COLUMNS {
                print!("{},", self.data.grapphic_array[(row * COLUMNS) + column]);
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
            print!("{:#02X?}, ", self.data.memory[i]);
        }
    }

    fn no_match(&mut self) {
        self.running = false;
        println!("Error: No matching opcode");
    }

    fn decode_opcode(&mut self) {
        let nibbles = (
            (self.data.opcode & 0xF000) >> 12,
            (self.data.opcode & 0x0F00) >> 8,
            (self.data.opcode & 0x00F0) >> 4,
            self.data.opcode & 0x000F
        );

        self.x = nibbles.1 as usize;
        self.y = nibbles.2 as usize;
        self.nnn = self.data.opcode & 0x0FFF;
        self.kk = (self.data.opcode & 0x00FF) as u8;
        self.n = nibbles.3 as usize;

        self.data.program_counter += PROGRAM_STEP;
        
        match nibbles {
            (0x0, 0x0, 0xe, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xe, 0xe) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(),
            (0x2, _, _, _) => self.op_2nnn(),
            (0x3, _, _, _) => self.op_3xkk(),
            (0x4, _, _, _) => self.op_4xkk(),
            (0x5, _, _, 0x0) => self.op_5xy0(),
            (0x6, _, _, _) => self.op_6xkk(),
            (0x7, _, _, _) => self.op_7xkk(),
            (0x8, _, _, 0x0) => self.op_8xy0(),
            (0x8, _, _, 0x1) => self.op_8xy1(),
            (0x8, _, _, 0x2) => self.op_8xy2(),
            (0x8, _, _, 0x3) => self.op_8xy3(),
            (0x8, _, _, 0x4) => self.op_8xy4(),
            (0x8, _, _, 0x5) => self.op_8xy5(),
            (0x8, _, _, 0x6) => self.op_8xy6(),
            (0x8, _, _, 0x7)=> self.op_8xy7(),
            (0x8, _, _, 0xE) => self.op_8xye(),
            (0x9, _, _, 0x0) => self.op_9xy0(),
            (0xA, _, _, _) => self.op_annn(),
            (0xB, _, _, _) => self.op_bnnn(),
            (0xC, _, _, _) => self.op_cxkk(),
            (0xD, _, _, _) => self.op_dxyn(),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(),
            (0xE, _, 0xA, 0x1) => self.op_exa1(),
            (0xF, _, 0x0, 0x7) => self.op_fx07(),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(),
            (0xF, _, 0x1, 0x5) => self.op_fx15(),
            (0xF, _, 0x1, 0x8) => self.op_fx18(),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(),
            (0xF, _, 0x2, 0x9) => self.op_fx29(),
            (0xF, _, 0x3, 0x3) => self.op_fx33(),
            (0xF, _, 0x5, 0x5) => self.op_fx55(),
            (0xF, _, 0x6, 0x5) => self.op_fx65(),
            _ => self.no_match(),
        }
        println!("");
    }

    //CLS
    fn op_00e0(&mut self) {
        for i in 0..self.data.grapphic_array.len() {
            self.data.grapphic_array[i] = 0;
        }
    }

    //RET from subroutine
    fn op_00ee(&mut self) {
        self.data.stack_pointer -= 1;
        self.data.program_counter = self.data.stack[self.data.stack_pointer] as usize;
        //self.stack[self.stack_pointer] = 0;
    }

    //JP addr
    fn op_1nnn(&mut self) {
        self.data.program_counter = self.nnn as usize;
    }
    
    //CALL addr
    fn op_2nnn(&mut self) {
        self.data.stack[self.data.stack_pointer] = self.data.program_counter as u16;
        self.data.stack_pointer += 1;
        self.data.program_counter = self.nnn as usize;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self) {
        if self.data.variable_register[self.x] == self.kk {
            self.data.program_counter += PROGRAM_STEP;
        }
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self) {
        if self.data.variable_register[self.x] != self.kk {
            self.data.program_counter += PROGRAM_STEP;
        }
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self) {
        if self.data.variable_register[self.x] == self.data.variable_register[self.y] {
            self.data.program_counter += PROGRAM_STEP;
        }
    }

    //LD Vx, byte
    fn op_6xkk(&mut self) {
        self.data.variable_register[self.x] = self.kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self) {
        self.data.variable_register[self.x] = self.data.variable_register[self.x].overflowing_add(self.kk).0;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self) {
        self.data.variable_register[self.x] = self.data.variable_register[self.y];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self) {
        self.data.variable_register[self.x] |= self.data.variable_register[self.y];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self) {
        self.data.variable_register[self.x] &= self.data.variable_register[self.y];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self) {
        self.data.variable_register[self.x] ^= self.data.variable_register[self.y];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self) {
        let (result, overflow) = self.data.variable_register[self.x].overflowing_add(self.data.variable_register[self.y]);
        self.data.variable_register[self.x] = result;
        self.data.variable_register[CARRY_FLAG] = overflow as u8;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self) {
        let (result, overflow) = self.data.variable_register[self.x].overflowing_sub(self.data.variable_register[self.y]);
        self.data.variable_register[self.x] = result;
        self.data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHR Vx
    fn op_8xy6(&mut self) {
        self.data.variable_register[CARRY_FLAG] = self.data.variable_register[self.x] & 0x1;
        self.data.variable_register[self.x] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self) {
        let (result, overflow) = self.data.variable_register[self.y].overflowing_sub(self.data.variable_register[self.x]);
        self.data.variable_register[self.x] = result;
        self.data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHL Vx
    fn op_8xye(&mut self) {
        self.data.variable_register[CARRY_FLAG] = self.data.variable_register[self.x] >> 7;
        self.data.variable_register[self.x] <<= 1;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self) {
        if self.data.variable_register[self.x] != self.data.variable_register[self.y] {
            self.data.program_counter += PROGRAM_STEP;
        }
    }

    //LD I, addr
    fn op_annn(&mut self) {
        self.data.index_register = self.nnn;
    }

    //JP V0, addr
    fn op_bnnn(&mut self) {
        self.data.program_counter = (self.nnn + self.data.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self) {
        let mut rng = rand::thread_rng();
        self.data.variable_register[self.x] = rng.gen_range(0..0xFF + 1) as u8 & self.kk;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self) {
        let mut x_coordinate : usize;
        let mut y_coordinate : usize;
        let mut sprite : u8; 

        self.data.variable_register[CARRY_FLAG] = 0;
        for row in 0..self.n as usize {
            y_coordinate = (self.data.variable_register[self.y] as usize + row) % ROWS;
            sprite = self.data.memory[self.data.index_register as usize + row];
            for column in 0..8 {
                x_coordinate = (self.data.variable_register[self.x] as usize + column) % COLUMNS;
                if (sprite & (0x80 >> column)) != 0 {
                    if self.data.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] == 1 {
                        self.data.variable_register[CARRY_FLAG] = 1;
                    }
                    self.data.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] ^= 1;
                }
            }
        }
    }

    //SKP Vx
    fn op_ex9e(&mut self) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(self.data.variable_register[self.x]) == 1 {
            self.data.program_counter += PROGRAM_STEP;
            keypad_borrow.reset_key(self.data.variable_register[self.x]);
        }
    }

     //SKNP Vx
     fn op_exa1(&mut self) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(self.data.variable_register[self.x]) == 0 {
            self.data.program_counter += PROGRAM_STEP;
        }
        keypad_borrow.reset_key(self.data.variable_register[self.x]);
    }

    //LD Vx, DT
    fn op_fx07(&mut self) {
        self.data.variable_register[self.x] = self.data.delay_timer;
    }

    //LD Vx, K
    fn op_fx0a(&mut self) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if let Some(key) = (*keypad_borrow).get_pressed_key() {
            self.data.variable_register[self.x] = key;
            keypad_borrow.reset_key(key);
        } else {
            self.data.program_counter -= PROGRAM_STEP;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self) {
        self.data.delay_timer = self.data.variable_register[self.x];
    }

    //LD ST, Vx
    fn op_fx18(&mut self) {
        self.data.sound_timer = self.data.variable_register[self.x];
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self) {
        self.data.index_register += self.data.variable_register[self.x] as u16;
    }

    //LD F, Vx
    fn op_fx29(&mut self) {
        self.data.index_register = (self.data.variable_register[self.x] as u16) * 0x5;
    }

    //LD B, Vx
    fn op_fx33(&mut self) {
        self.data.memory[self.data.index_register as usize] = self.data.variable_register[self.x] / 100;
        self.data.memory[self.data.index_register as usize + 1] = (self.data.variable_register[self.x] / 10) % 10;
        self.data.memory[self.data.index_register as usize + 2] = self.data.variable_register[self.x] % 10;

    }

    //LD [I], Vx
    fn op_fx55(&mut self) {
        for i in 0..self.x + 1 {
            self.data.memory[self.data.index_register as usize + i] = self.data.variable_register[i];
        }

    }    

    //LD Vx, [I]
    fn op_fx65(&mut self) {
        if self.data.index_register as usize + self.x < MEMORYSIZE {
            for i in 0..self.x + 1{
                self.data.variable_register[i] = self.data.memory[self.data.index_register as usize + i];
            }
        }
    }   
}




