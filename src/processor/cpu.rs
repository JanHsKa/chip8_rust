use crate::keypad::Keypad;
use crate::processor::{memory_constants, FONTSET, Memory, MemoryAccess};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::option::Option;

use rand::Rng;

use self::memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE};


pub struct Cpu {
    data_ref: Rc<RefCell<Memory>>,
    keypad: Rc<RefCell<Keypad>>,
    running: bool,
    x: usize,
    y: usize,
    nnn: u16,
    kk: u8,
    n: usize,
}

impl Cpu {
    pub fn new(new_keypad: Rc<RefCell<Keypad>>, new_data: Rc<RefCell<Memory>>) -> Cpu {
        new_data.borrow_mut().memory[..FONTSET.len()].copy_from_slice(&FONTSET[..]);
        Cpu{
            data_ref: new_data,
            keypad: new_keypad,
            running: true,
            x: 0,
            y: 0,
            nnn: 0,
            kk: 0,
            n: 0,
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn load_program_code(&mut self, code: [u8; MAX_PROGRAM_SIZE]) {
        let mut data = self.data_ref.borrow_mut();
        for i in 0..MAX_PROGRAM_SIZE {
            data.memory[i + PROGRAM_START] = code[i];
        }
    }

    fn set_opcode(&mut self, mut data: RefMut<Memory>) {
        data.opcode = (data.memory[data.program_counter] as u16) << 8 
            | (data.memory[data.program_counter + 1] as u16);
    }

    pub fn get_state(&mut self) -> bool {
        return self.running;
    }


    pub fn run_opcode(&mut self) {
        if self.running {
            let mut data = self.data_ref.borrow_mut();
            self.set_opcode(data);
            //self.print_memory();
            self.decode_opcode(data);
        }
    }

    fn print_memory(&mut self) {
        //println!("opcode: {:#04X?}", self.data.opcode);
        //println!("program counter: {:#04X?}", self.data.program_counter);
    }

    pub fn tick_timer(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        if self.running {
            data.delay_timer = data.delay_timer.saturating_sub(1);
            
            if data.sound_timer > 0 {
                data.sound_timer -= 1;
            }
        }
    }

    pub fn get_graphic_array(&mut self) -> [u8; GRAPHIC_SIZE] {
        self.data_ref.borrow_mut().grapphic_array.clone()
    }
    pub fn play_sound(&mut self) -> bool {
        self.data_ref.borrow_mut().sound_timer > 0
    }

    fn print_graphic_array(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        for row in 0..ROWS as usize {
            for column in 0..COLUMNS {
                print!("{},", data.grapphic_array[(row * COLUMNS) + column]);
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
            print!("{:#02X?}, ", self.data_ref.borrow_mut().memory[i]);
        }
    }

    fn no_match(&mut self) {
        self.running = false;
        println!("Error: No matching opcode");
    }

    fn decode_opcode(&mut self, mut data: RefMut<Memory>) {
        let nibbles = (
            (data.opcode & 0xF000) >> 12,
            (data.opcode & 0x0F00) >> 8,
            (data.opcode & 0x00F0) >> 4,
            data.opcode & 0x000F
        );

        self.x = nibbles.1 as usize;
        self.y = nibbles.2 as usize;
        self.nnn = data.opcode & 0x0FFF;
        self.kk = (data.opcode & 0x00FF) as u8;
        self.n = nibbles.3 as usize;

        data.program_counter += PROGRAM_STEP;
        
        match nibbles {
            (0x0, 0x0, 0xe, 0x0) => self.op_00e0(data),
            (0x0, 0x0, 0xe, 0xe) => self.op_00ee(data),
            (0x1, _, _, _) => self.op_1nnn(data),
            (0x2, _, _, _) => self.op_2nnn(data),
            (0x3, _, _, _) => self.op_3xkk(data),
            (0x4, _, _, _) => self.op_4xkk(data),
            (0x5, _, _, 0x0) => self.op_5xy0(data),
            (0x6, _, _, _) => self.op_6xkk(data),
            (0x7, _, _, _) => self.op_7xkk(data),
            (0x8, _, _, 0x0) => self.op_8xy0(data),
            (0x8, _, _, 0x1) => self.op_8xy1(data),
            (0x8, _, _, 0x2) => self.op_8xy2(data),
            (0x8, _, _, 0x3) => self.op_8xy3(data),
            (0x8, _, _, 0x4) => self.op_8xy4(data),
            (0x8, _, _, 0x5) => self.op_8xy5(data),
            (0x8, _, _, 0x6) => self.op_8xy6(data),
            (0x8, _, _, 0x7)=> self.op_8xy7(data),
            (0x8, _, _, 0xE) => self.op_8xye(data),
            (0x9, _, _, 0x0) => self.op_9xy0(data),
            (0xA, _, _, _) => self.op_annn(data),
            (0xB, _, _, _) => self.op_bnnn(data),
            (0xC, _, _, _) => self.op_cxkk(data),
            (0xD, _, _, _) => self.op_dxyn(data),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(data),
            (0xE, _, 0xA, 0x1) => self.op_exa1(data),
            (0xF, _, 0x0, 0x7) => self.op_fx07(data),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(data),
            (0xF, _, 0x1, 0x5) => self.op_fx15(data),
            (0xF, _, 0x1, 0x8) => self.op_fx18(data),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(data),
            (0xF, _, 0x2, 0x9) => self.op_fx29(data),
            (0xF, _, 0x3, 0x3) => self.op_fx33(data),
            (0xF, _, 0x5, 0x5) => self.op_fx55(data),
            (0xF, _, 0x6, 0x5) => self.op_fx65(data),
            _ => self.no_match(),
        }
        println!("");
    }

    //CLS
    fn op_00e0(&mut self, mut data: RefMut<Memory>) {
        for i in 0..data.grapphic_array.len() {
            data.grapphic_array[i] = 0;
        }
    }

    //RET from subroutine
    fn op_00ee(&mut self, mut data: RefMut<Memory>) {
        data.stack_pointer -= 1;
        data.program_counter = data.stack[data.stack_pointer] as usize;
        //self.stack[self.stack_pointer] = 0;
    }

    //JP addr
    fn op_1nnn(&mut self, mut data: RefMut<Memory>) {
        data.program_counter = self.nnn as usize;
    }
    
    //CALL addr
    fn op_2nnn(&mut self, mut data: RefMut<Memory>) {
        data.stack[data.stack_pointer] = data.program_counter as u16;
        data.stack_pointer += 1;
        data.program_counter = self.nnn as usize;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self, mut data: RefMut<Memory>) {
        if data.variable_register[self.x] == self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self, mut data: RefMut<Memory>) {
        if data.variable_register[self.x] != self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self, mut data: RefMut<Memory>) {
        if data.variable_register[self.x] == data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD Vx, byte
    fn op_6xkk(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] = self.kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] = data.variable_register[self.x].overflowing_add(self.kk).0;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] = data.variable_register[self.y];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] |= data.variable_register[self.y];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] &= data.variable_register[self.y];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] ^= data.variable_register[self.y];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self, mut data: RefMut<Memory>) {
        let (result, overflow) = data.variable_register[self.x].overflowing_add(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = overflow as u8;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self, mut data: RefMut<Memory>) {
        let (result, overflow) = data.variable_register[self.x].overflowing_sub(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHR Vx
    fn op_8xy6(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] & 0x1;
        data.variable_register[self.x] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self, mut data: RefMut<Memory>) {
        let (result, overflow) = data.variable_register[self.y].overflowing_sub(data.variable_register[self.x]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHL Vx
    fn op_8xye(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] >> 7;
        data.variable_register[self.x] <<= 1;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self, mut data: RefMut<Memory>) {
        if data.variable_register[self.x] != data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD I, addr
    fn op_annn(&mut self, mut data: RefMut<Memory>) {
        data.index_register = self.nnn;
    }

    //JP V0, addr
    fn op_bnnn(&mut self, mut data: RefMut<Memory>) {
        data.program_counter = (self.nnn + data.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self, mut data: RefMut<Memory>) {
        let mut rng = rand::thread_rng();
        data.variable_register[self.x] = rng.gen_range(0..0xFF + 1) as u8 & self.kk;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, mut data: RefMut<Memory>) {
        let mut x_coordinate : usize;
        let mut y_coordinate : usize;
        let mut sprite : u8; 

        data.variable_register[CARRY_FLAG] = 0;
        for row in 0..self.n as usize {
            y_coordinate = (data.variable_register[self.y] as usize + row) % ROWS;
            sprite = data.memory[data.index_register as usize + row];
            for column in 0..8 {
                x_coordinate = (data.variable_register[self.x] as usize + column) % COLUMNS;
                if (sprite & (0x80 >> column)) != 0 {
                    if data.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] == 1 {
                        data.variable_register[CARRY_FLAG] = 1;
                    }
                data.grapphic_array[(y_coordinate * COLUMNS) + x_coordinate] ^= 1;
                }
            }
        }
    }

    //SKP Vx
    fn op_ex9e(&mut self, mut data: RefMut<Memory>) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(data.variable_register[self.x]) == 1 {
            data.program_counter += PROGRAM_STEP;
            keypad_borrow.reset_key(data.variable_register[self.x]);
        }
    }

     //SKNP Vx
     fn op_exa1(&mut self, mut data: RefMut<Memory>) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(data.variable_register[self.x]) == 0 {
            data.program_counter += PROGRAM_STEP;
        }
        keypad_borrow.reset_key(data.variable_register[self.x]);
    }

    //LD Vx, DT
    fn op_fx07(&mut self, mut data: RefMut<Memory>) {
        data.variable_register[self.x] = data.delay_timer;
    }

    //LD Vx, K
    fn op_fx0a(&mut self, mut data: RefMut<Memory>) {
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if let Some(key) = (*keypad_borrow).get_pressed_key() {
            data.variable_register[self.x] = key;
            keypad_borrow.reset_key(key);
        } else {
            data.program_counter -= PROGRAM_STEP;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self, mut data: RefMut<Memory>) {
        data.delay_timer = data.variable_register[self.x];
    }

    //LD ST, Vx
    fn op_fx18(&mut self, mut data: RefMut<Memory>) {
        data.sound_timer = data.variable_register[self.x];
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self, mut data: RefMut<Memory>) {
        data.index_register += data.variable_register[self.x] as u16;
    }

    //LD F, Vx
    fn op_fx29(&mut self, mut data: RefMut<Memory>) {
        data.index_register = (data.variable_register[self.x] as u16) * 0x5;
    }

    //LD B, Vx
    fn op_fx33(&mut self, mut data: RefMut<Memory>) {
        data.memory[data.index_register as usize] = data.variable_register[self.x] / 100;
        data.memory[data.index_register as usize + 1] = (data.variable_register[self.x] / 10) % 10;
        data.memory[data.index_register as usize + 2] = data.variable_register[self.x] % 10;

    }

    //LD [I], Vx
    fn op_fx55(&mut self, mut data: RefMut<Memory>) {
        for i in 0..self.x + 1 {
            data.memory[data.index_register as usize + i] = data.variable_register[i];
        }

    }    

    //LD Vx, [I]
    fn op_fx65(&mut self, mut data: RefMut<Memory>) {
        if data.index_register as usize + self.x < MEMORYSIZE {
            for i in 0..self.x + 1{
                data.variable_register[i] = data.memory[data.index_register as usize + i];
            }
        }
    }   
}




