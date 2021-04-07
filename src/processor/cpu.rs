use crate::utils::{Keypad};
use crate::processor::{memory_constants, FONTSET, Memory};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use rand::Rng;

use self::memory_constants::{
    MEMORYSIZE, VARIABLES_COUNT, COLUMNS, 
    ROWS, STACKSIZE, CARRY_FLAG, 
    MAX_PROGRAM_SIZE, PROGRAM_START, 
    PROGRAM_STEP, GRAPHIC_SIZE};

struct Nibbles (u16, u16, u16, u16);

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
        self.running = true;
        self.data_ref.borrow_mut().reset();
        self.data_ref.borrow_mut().memory[..FONTSET.len()].copy_from_slice(&FONTSET[..]);

    }

    pub fn load_program_code(&mut self, code: [u8; MAX_PROGRAM_SIZE]) {
        let mut data = self.data_ref.borrow_mut();
        data.memory[PROGRAM_START..MEMORYSIZE].copy_from_slice(&code[..MAX_PROGRAM_SIZE]);
    }

    fn set_opcode(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.opcode = (data.memory[data.program_counter] as u16) << 8 
            | (data.memory[data.program_counter + 1] as u16);
    }

    pub fn get_state(&mut self) -> bool {
        return self.running;
    }


    pub fn run_opcode(&mut self) {
        if self.running {
            self.set_opcode();
            let nibbles = self.decode_opcode();
            self.match_opcode(nibbles);
        }
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

    fn no_match(&mut self) {
        self.running = false;
        println!("Error: No matching opcode");
    }

    fn decode_opcode(&mut self) -> (u16, u16, u16, u16) {
        let mut data = self.data_ref.borrow_mut();

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
        
        nibbles
    }

    fn match_opcode(&mut self, nibbles: (u16, u16, u16, u16)) {
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
    }
    //CLS
    fn op_00e0(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        for i in 0..data.grapphic_array.len() {
            data.grapphic_array[i] = 0;
        }
    }

    //RET from subroutine
    fn op_00ee(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.stack_pointer -= 1;
        let stack_pointer = data.stack_pointer;
        data.program_counter = data.stack[stack_pointer] as usize;
        data.stack[stack_pointer] = 0;
    }

    //JP addr
    fn op_1nnn(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.program_counter = self.nnn as usize;
    }
    
    //CALL addr
    fn op_2nnn(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let stack_pointer = data.stack_pointer;
        data.stack[stack_pointer] = data.program_counter as u16;
        data.stack_pointer += 1;
        data.program_counter = self.nnn as usize;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        if data.variable_register[self.x] == self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        if data.variable_register[self.x] != self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        if data.variable_register[self.x] == data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD Vx, byte
    fn op_6xkk(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] = self.kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] = data.variable_register[self.x].overflowing_add(self.kk).0;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] = data.variable_register[self.y];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] |= data.variable_register[self.y];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] &= data.variable_register[self.y];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] ^= data.variable_register[self.y];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let (result, overflow) = data.variable_register[self.x].overflowing_add(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = overflow as u8;
    }
    
    //SUB Vx, Vy
    fn op_8xy5(&mut self) {
        let mut data = self.data_ref.borrow_mut();

        let (result, overflow) = data.variable_register[self.x].overflowing_sub(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHR Vx
    fn op_8xy6(&mut self) {
        let mut data = self.data_ref.borrow_mut();

        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] & 0x1;
        data.variable_register[self.x] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let (result, overflow) = data.variable_register[self.y].overflowing_sub(data.variable_register[self.x]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHL Vx
    fn op_8xye(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] >> 7;
        data.variable_register[self.x] <<= 1;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        if data.variable_register[self.x] != data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD I, addr
    fn op_annn(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.index_register = self.nnn;
    }

    //JP V0, addr
    fn op_bnnn(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.program_counter = (self.nnn + data.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let mut rng = rand::thread_rng();
        data.variable_register[self.x] = rng.gen_range(0..0xFF + 1) as u8 & self.kk;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self) {
        let mut data = self.data_ref.borrow_mut();
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
    fn op_ex9e(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(data.variable_register[self.x]) == 1 {
            data.program_counter += PROGRAM_STEP;
            keypad_borrow.reset_key(data.variable_register[self.x]);
        }
    }

     //SKNP Vx
     fn op_exa1(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if keypad_borrow.get_key(data.variable_register[self.x]) == 0 {
            data.program_counter += PROGRAM_STEP;
        }
        keypad_borrow.reset_key(data.variable_register[self.x]);
    }

    //LD Vx, DT
    fn op_fx07(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.variable_register[self.x] = data.delay_timer;
    }

    //LD Vx, K
    fn op_fx0a(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let mut keypad_borrow :RefMut<Keypad> = self.keypad.borrow_mut();
        if let Some(key) = (*keypad_borrow).get_pressed_key() {
            data.variable_register[self.x] = key;
            keypad_borrow.reset_key(key);
        } else {
            data.program_counter -= PROGRAM_STEP;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.delay_timer = data.variable_register[self.x];
    }

    //LD ST, Vx
    fn op_fx18(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.sound_timer = data.variable_register[self.x];
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.index_register += data.variable_register[self.x] as u16;
    }

    //LD F, Vx
    fn op_fx29(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        data.index_register = (data.variable_register[self.x] as u16) * 0x5;
    }

    //LD B, Vx
    fn op_fx33(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let index = data.index_register as usize;
        data.memory[index] = data.variable_register[self.x] / 100;
        data.memory[index + 1] = (data.variable_register[self.x] / 10) % 10;
        data.memory[index + 2] = data.variable_register[self.x] % 10;

    }

    //LD [I], Vx
    fn op_fx55(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let index = data.index_register as usize;
        for i in 0..self.x + 1 {
            data.memory[index + i] = data.variable_register[i];
        }

    }    

    //LD Vx, [I]
    fn op_fx65(&mut self) {
        let mut data = self.data_ref.borrow_mut();
        let index = data.index_register as usize;
        if index + self.x < MEMORYSIZE {
            for i in 0..self.x + 1{
                data.variable_register[i] = data.memory[index + i];
            }
        }
    }   
}




