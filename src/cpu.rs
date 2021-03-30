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
use constants::PROGRAM_STEP;


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
    x: usize,
    y: usize,
    nnn: u16,
    kk: u8,
    n: usize,
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
            x: 0,
            y: 0,
            nnn: 0,
            kk: 0,
            n: 0,
        };

        cpu.memory[..fontset::FONTSET.len()].copy_from_slice(&fontset::FONTSET[..]);
        
        cpu
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
        if self.running {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }
    }

    pub fn play_sound(&mut self) -> bool {
        self.sound_timer > 0
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
        let nibbles = (
            (self.opcode & 0xF000) >> 12,
            (self.opcode & 0x0F00) >> 8,
            (self.opcode & 0x00F0) >> 4,
            self.opcode & 0x000F
        );

        self.x = nibbles.1 as usize;
        self.y = nibbles.2 as usize;
        self.nnn = self.opcode & 0x0FFF;
        self.kk = (self.opcode & 0x00FF) as u8;
        self.n = nibbles.3 as usize;

        self.program_counter += PROGRAM_STEP;
        println!("opcode: {:04X?}", self.opcode);
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
        println!("funtion: op_00e0");

        for i in 0..self.grapphic_array.len() {
            self.grapphic_array[i] = 0;
        }
    }

    //RET from subroutine
    fn op_00ee(&mut self) {
        println!("funtion: op_00ee");
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
        //self.stack[self.stack_pointer] = 0;
    }

    //JP addr
    fn op_1nnn(&mut self) {
        println!("funtion: op_1nnn");
        self.program_counter = self.nnn as usize;
    }
    
    //CALL addr
    fn op_2nnn(&mut self) {
        println!("funtion: op_2nnn");
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = self.nnn as usize;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self) {
        println!("funtion: op_3xkk");
        if self.variable_register[self.x] == self.kk {
            self.program_counter += PROGRAM_STEP;
        }
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self) {
        println!("funtion: op_4xkk");
        if self.variable_register[self.x] != self.kk {
            self.program_counter += PROGRAM_STEP;
        }
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self) {
        println!("funtion: op_5xy0");
        if self.variable_register[self.x] == self.variable_register[self.y] {
            self.program_counter += PROGRAM_STEP;
        }
    }

    //LD Vx, byte
    fn op_6xkk(&mut self) {
        println!("funtion: op_6xkk");
        self.variable_register[self.x] = self.kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self) {
        println!("funtion: op_7xkk");
        self.variable_register[self.x] = self.variable_register[self.x].overflowing_add(self.kk).0;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self) {
        println!("funtion: op_8xy0");
        self.variable_register[self.x] = self.variable_register[self.y];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self) {
        println!("funtion: op_8xy1");
        self.variable_register[self.x] |= self.variable_register[self.y];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self) {
        println!("funtion: op_8xy2");
        self.variable_register[self.x] &= self.variable_register[self.y];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self) {
        println!("funtion: op_8xy3");
        self.variable_register[self.x] ^= self.variable_register[self.y];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self) {
        println!("funtion: op_8xy4");
        let (result, overflow) = self.variable_register[self.x].overflowing_add(self.variable_register[self.y]);
        self.variable_register[self.x] = result;
        self.variable_register[CARRY_FLAG] = overflow as u8;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self) {
        println!("funtion: op_8xy5");
        let (result, overflow) = self.variable_register[self.x].overflowing_sub(self.variable_register[self.y]);
        self.variable_register[self.x] = result;
        self.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHR Vx
    fn op_8xy6(&mut self) {
        println!("funtion: op_8xy6");
        self.variable_register[CARRY_FLAG] = self.variable_register[self.x] & 0x1;
        self.variable_register[self.x] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self) {
        println!("funtion: op_8xy7");
        let (result, overflow) = self.variable_register[self.y].overflowing_sub(self.variable_register[self.x]);
        self.variable_register[self.x] = result;
        self.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHL Vx
    fn op_8xye(&mut self) {
        println!("funtion: op_8xye");
        self.variable_register[CARRY_FLAG] = self.variable_register[self.x] >> 7;
        self.variable_register[self.x] <<= 1;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self) {
        println!("funtion: op_9xy0");
        if self.variable_register[self.x] != self.variable_register[self.y] {
            self.program_counter += PROGRAM_STEP;
        }
    }

    //LD I, addr
    fn op_annn(&mut self) {
        println!("funtion: op_annn");
        self.index_register = self.nnn;
    }

    //JP V0, addr
    fn op_bnnn(&mut self) {
        println!("funtion: op_bnnn");
        self.program_counter = (self.nnn + self.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self) {
        println!("funtion: op_cxkk");
        let mut rng = rand::thread_rng();
        self.variable_register[self.x] = rng.gen_range(0..0xFF + 1) as u8 & self.kk;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self) {
        println!("funtion: op_dxyn");
        let mut x_coordinate : usize;
        let mut y_coordinate : usize;
        let mut sprite : u8; 

        self.variable_register[CARRY_FLAG] = 0;
        for row in 0..self.n as usize {
            y_coordinate = (self.variable_register[self.y] as usize + row) % ROWS;
            sprite = self.memory[self.index_register as usize + row];
            for column in 0..8 {
                x_coordinate = (self.variable_register[self.x] as usize + column) % COLUMNS;
                if (sprite & (0x80 >> column)) != 0 {
                    if self.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] == 1 {
                        self.variable_register[CARRY_FLAG] = 1;
                    }
                    self.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] ^= 1;
                }
            }
        }
    }

    //SKP Vx
    fn op_ex9e(&mut self) {
        println!("funtion: op_ex9e");
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(self.variable_register[self.x]) == 1 {
            self.program_counter += PROGRAM_STEP;
            keypad_borrow.reset_key(self.variable_register[self.x]);
        }
    }

     //SKNP Vx
     fn op_exa1(&mut self) {
        println!("funtion: op_exa1");
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(self.variable_register[self.x]) == 0 {
            self.program_counter += PROGRAM_STEP;
        }
        keypad_borrow.reset_key(self.variable_register[self.x]);
    }

    //LD Vx, DT
    fn op_fx07(&mut self) {
        println!("funtion: op_fx07");
        self.variable_register[self.x] = self.delay_timer;
    }

    //LD Vx, K
    fn op_fx0a(&mut self) {
        println!("funtion: op_fx0a");
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if let Some(key) = (*keypad_borrow).get_pressed_key() {
            self.variable_register[self.x] = key;
            keypad_borrow.reset_key(key);
        } else {
            self.program_counter -= PROGRAM_STEP;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self) {
        println!("funtion: op_fx15");
        self.delay_timer = self.variable_register[self.x];
    }

    //LD ST, Vx
    fn op_fx18(&mut self) {
        println!("funtion: op_fx18");
        self.sound_timer = self.variable_register[self.x];
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self) {
        println!("funtion: op_fx1e");
        self.index_register += self.variable_register[self.x] as u16;
    }

    //LD F, Vx
    fn op_fx29(&mut self) {
        println!("funtion: op_fx29");
        self.index_register = (self.variable_register[self.x] as u16) * 0x5;
    }

    //LD B, Vx
    fn op_fx33(&mut self) {
        println!("funtion: op_fx33");
        self.memory[self.index_register as usize] = self.variable_register[self.x] / 100;
        self.memory[self.index_register as usize + 1] = (self.variable_register[self.x] / 10) % 10;
        self.memory[self.index_register as usize + 2] = self.variable_register[self.x] % 10;

    }

    //LD [I], Vx
    fn op_fx55(&mut self) {
        println!("funtion: op_fx55");
        for i in 0..self.x + 1 {
            self.memory[self.index_register as usize + i] = self.variable_register[i];
        }

    }    

    //LD Vx, [I]
    fn op_fx65(&mut self) {
        println!("funtion: op_fx65");
        if self.index_register as usize + self.x < MEMORYSIZE {
            for i in 0..self.x + 1{
                self.variable_register[i] = self.memory[self.index_register as usize + i];
            }
        }
    }   
}




