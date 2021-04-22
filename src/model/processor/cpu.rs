use crate::defines::{
    memory_constants::{
        BIG_SPRITE, CARRY_FLAG, COLUMNS, FLAG_REGISTER_SIZE, GRAPHIC_SIZE, GRAPHIC_SIZE_HIGH,
        MAX_PROGRAM_SIZE, MEMORYSIZE, PROGRAM_START, PROGRAM_STEP, ROWS, SCROLL_RANGE,
        SPRITE_WIDTH, STACKSIZE, VARIABLES_COUNT,
    },
    CpuState, KeyPress, Reset,
};

use crate::model::{
    Keypad, Memory, Resolution, FONTSET_HIGH, FONTSET_HIGH_SIZE, FONTSET_HIGH_START, FONTSET_LOW,
    FONTSET_LOW_SIZE,
};

use rand::Rng;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Copy, Clone, PartialEq)]
pub struct BitState;

impl BitState {
    pub const UNSET: u8 = 0;
    pub const SET: u8 = 1;
}

pub struct Cpu {
    data_ref: Arc<Mutex<Memory>>,
    keypad: Arc<Mutex<Keypad>>,
    running: CpuState,
    x: usize,
    y: usize,
    nnn: u16,
    kk: u8,
    n: usize,
    max_columns: usize,
    max_rows: usize,
}

impl Cpu {
    pub fn new(new_keypad: Arc<Mutex<Keypad>>, new_data: Arc<Mutex<Memory>>) -> Cpu {
        new_data.lock().unwrap().memory[..FONTSET_LOW_SIZE].copy_from_slice(&FONTSET_LOW[..]);
        new_data.lock().unwrap().memory[FONTSET_HIGH_START..FONTSET_HIGH_START + FONTSET_HIGH_SIZE]
            .copy_from_slice(&FONTSET_HIGH[..]);

        Cpu {
            data_ref: new_data,
            keypad: new_keypad,
            running: CpuState::Running,
            x: 0,
            y: 0,
            nnn: 0,
            kk: 0,
            n: 0,
            max_columns: COLUMNS,
            max_rows: ROWS,
        }
    }

    pub fn reset(&mut self) {
        self.running = CpuState::Running;
        self.max_columns = COLUMNS;
        self.max_rows = ROWS;

        let mut data = self.data_ref.lock().unwrap();
        data.reset();
        data.memory[..FONTSET_LOW_SIZE].copy_from_slice(&FONTSET_LOW[..]);
        data.memory[FONTSET_HIGH_START..FONTSET_HIGH_START + FONTSET_HIGH_SIZE]
            .copy_from_slice(&FONTSET_HIGH[..]);
    }

    pub fn load_program_code(&mut self, code: &[u8]) {
        let mut data = self.data_ref.lock().unwrap();
        let code_size = code.len();
        data.memory[PROGRAM_START..PROGRAM_START + code_size].copy_from_slice(&code);
    }

    fn set_opcode(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.opcode = (data.memory[data.program_counter] as u16) << 8
            | (data.memory[data.program_counter + 1] as u16);
    }

    pub fn get_state(&mut self) -> CpuState {
        self.running
    }

    pub fn run_opcode(&mut self) {
        if self.running == CpuState::Running {
            self.set_opcode();
            let nibbles = self.decode_opcode();
            self.match_opcode(nibbles);
        }
    }

    pub fn tick_timer(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        if self.running == CpuState::Running {
            data.delay_timer = data.delay_timer.saturating_sub(1);

            if data.sound_timer > 0 {
                data.sound_timer -= 1;
            }
        }
    }

    pub fn play_sound(&mut self) -> bool {
        self.data_ref.lock().unwrap().sound_timer > 0
    }

    fn print_graphic_array(&mut self) {
        let graphic_array = self.data_ref.lock().unwrap().graphic_array.clone();

        for (i, iter) in graphic_array.iter().enumerate() {
            if i % self.max_columns == 0 {
                println!("");
            }
            print!("{}", iter);
        }
    }

    fn no_match(&mut self) {
        self.running = CpuState::Stopped;
        println!(
            "Error: No matching opcode: {:04X}",
            self.data_ref.lock().unwrap().opcode
        );
    }

    fn decode_opcode(&mut self) -> (u16, u16, u16, u16) {
        let mut data = self.data_ref.lock().unwrap();

        let nibbles = (
            (data.opcode & 0xF000) >> 12,
            (data.opcode & 0x0F00) >> 8,
            (data.opcode & 0x00F0) >> 4,
            data.opcode & 0x000F,
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
            (0x0, 0x0, 0xb, _) => self.op_00bn(),
            (0x0, 0x0, 0xc, _) => self.op_00cn(),
            (0x0, 0x0, 0xe, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xe, 0xe) => self.op_00ee(),
            (0x0, 0x0, 0xf, 0xb) => self.op_00fb(),
            (0x0, 0x0, 0xf, 0xc) => self.op_00fc(),
            (0x0, 0x0, 0xf, 0xd) => self.op_00fd(),
            (0x0, 0x0, 0xf, 0xe) => self.op_00fe(),
            (0x0, 0x0, 0xf, 0xf) => self.op_00ff(),
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
            (0x8, _, _, 0x7) => self.op_8xy7(),
            (0x8, _, _, 0xE) => self.op_8xye(),
            (0x9, _, _, 0x0) => self.op_9xy0(),
            (0xA, _, _, _) => self.op_annn(),
            (0xB, _, _, _) => self.op_bnnn(),
            (0xC, _, _, _) => self.op_cxkk(),
            (0xD, _, _, 0x0) => self.op_dxy0(),
            (0xD, _, _, _) => self.op_dxyn(),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(),
            (0xE, _, 0xA, 0x1) => self.op_exa1(),
            (0xF, _, 0x0, 0x7) => self.op_fx07(),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(),
            (0xF, _, 0x1, 0x5) => self.op_fx15(),
            (0xF, _, 0x1, 0x8) => self.op_fx18(),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(),
            (0xF, _, 0x2, 0x9) => self.op_fx29(),
            (0xF, _, 0x3, 0x0) => self.op_fx30(),
            (0xF, _, 0x3, 0x3) => self.op_fx33(),
            (0xF, _, 0x5, 0x5) => self.op_fx55(),
            (0xF, _, 0x6, 0x5) => self.op_fx65(),
            (0xF, _, 0x7, 0x5) => self.op_fx75(),
            (0xF, _, 0x8, 0x5) => self.op_fx85(),
            _ => self.no_match(),
        }
    }

    //Scroll Up
    fn op_00bn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let graphic_copy = data.graphic_array.clone();
        let graphic_size = graphic_copy.len();
        data.graphic_array.reset_all();
        let shift: usize = self.n * COLUMNS * data.resolution as usize;
        data.graphic_array[..graphic_size - shift].copy_from_slice(&graphic_copy[shift..]);
    }

    //Scroll Down
    fn op_00cn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let graphic_copy = data.graphic_array.clone();
        data.graphic_array.reset_all();
        let graphic_size = graphic_copy.len();
        let shift: usize = self.n * COLUMNS * data.resolution as usize;
        data.graphic_array[shift..].copy_from_slice(&graphic_copy[..graphic_size - shift]);
    }

    //CLS
    fn op_00e0(&mut self) {
        self.data_ref.lock().unwrap().graphic_array.reset_all();
    }

    //RET from subroutine
    fn op_00ee(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.stack_pointer -= 1;
        let stack_pointer = data.stack_pointer;
        data.program_counter = data.stack[stack_pointer] as usize;
        data.stack[stack_pointer] = 0;
    }

    //Scroll Right
    fn op_00fb(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let graphic_copy = data.graphic_array.clone();

        for (i, pixel) in data.graphic_array.iter_mut().enumerate() {
            if i % self.max_columns < SCROLL_RANGE + 1 {
                *pixel = 0;
            } else {
                *pixel = graphic_copy[i - SCROLL_RANGE];
            }
        }
    }

    //Scroll Left
    fn op_00fc(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let graphic_copy = data.graphic_array.clone();

        for (i, pixel) in data.graphic_array.iter_mut().enumerate() {
            if i % self.max_columns > self.max_columns - SCROLL_RANGE
                || i + SCROLL_RANGE >= graphic_copy.len()
            {
                *pixel = 0;
            } else {
                *pixel = graphic_copy[i + SCROLL_RANGE];
            }
        }
    }

    //Exit
    fn op_00fd(&mut self) {
        println!("Exit");
        self.running = CpuState::Stopped;
    }

    //Low Res
    fn op_00fe(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let data_copy = data.graphic_array.clone();
        let res_factor = data.resolution as usize;
        data.resolution = Resolution::Low;
        data.graphic_array = vec![0; GRAPHIC_SIZE];
        self.max_columns = COLUMNS * Resolution::Low as usize;
        self.max_rows = ROWS * Resolution::Low as usize;

        for x in (0..self.max_columns).step_by(res_factor) {
            for y in (0..self.max_rows).step_by(res_factor) {
                data.graphic_array[(x / res_factor) + (y / res_factor) * COLUMNS] =
                    data_copy[x + y * self.max_columns];
            }
        }
    }

    //High Res
    fn op_00ff(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let data_copy = data.graphic_array.clone();
        let res_factor = Resolution::High as usize;
        data.resolution = Resolution::High;
        data.graphic_array = vec![0; GRAPHIC_SIZE_HIGH];
        self.max_columns = COLUMNS * Resolution::High as usize;
        self.max_rows = ROWS * Resolution::High as usize;

        for x in 0..self.max_columns {
            for y in 0..self.max_rows {
                data.graphic_array[x + y * self.max_columns] =
                    data_copy[(x / res_factor) + (y / res_factor) * COLUMNS];
            }
        }
    }

    //JP addr
    fn op_1nnn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.program_counter = self.nnn as usize;
    }

    //CALL addr
    fn op_2nnn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let stack_pointer = data.stack_pointer;
        data.stack[stack_pointer] = data.program_counter as u16;
        data.stack_pointer += 1;
        data.program_counter = self.nnn as usize;
    }

    //SE Vx, byte
    fn op_3xkk(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        if data.variable_register[self.x] == self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SNE Vx, byte
    fn op_4xkk(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        if data.variable_register[self.x] != self.kk {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //SE Vx, Vy
    fn op_5xy0(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        if data.variable_register[self.x] == data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD Vx, byte
    fn op_6xkk(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] = self.kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] = data.variable_register[self.x].overflowing_add(self.kk).0;
    }

    //LD Vx, Vy
    fn op_8xy0(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] = data.variable_register[self.y];
    }

    //OR Vx, Vy
    fn op_8xy1(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] |= data.variable_register[self.y];
    }

    //AND Vx, Vy
    fn op_8xy2(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] &= data.variable_register[self.y];
    }

    //XOR Vx, Vy
    fn op_8xy3(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] ^= data.variable_register[self.y];
    }

    //ADD Vx, Vy
    fn op_8xy4(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let (result, overflow) =
            data.variable_register[self.x].overflowing_add(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = overflow as u8;
    }

    //SUB Vx, Vy
    fn op_8xy5(&mut self) {
        let mut data = self.data_ref.lock().unwrap();

        let (result, overflow) =
            data.variable_register[self.x].overflowing_sub(data.variable_register[self.y]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHR Vx
    fn op_8xy6(&mut self) {
        let mut data = self.data_ref.lock().unwrap();

        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] & 0x1;
        data.variable_register[self.x] >>= 1;
    }

    //SUBN Vx, Vy
    fn op_8xy7(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let (result, overflow) =
            data.variable_register[self.y].overflowing_sub(data.variable_register[self.x]);
        data.variable_register[self.x] = result;
        data.variable_register[CARRY_FLAG] = !overflow as u8;
    }

    //SHL Vx
    fn op_8xye(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[CARRY_FLAG] = data.variable_register[self.x] >> 7;
        data.variable_register[self.x] <<= 1;
    }

    //SNE Vx, Vy
    fn op_9xy0(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        if data.variable_register[self.x] != data.variable_register[self.y] {
            data.program_counter += PROGRAM_STEP;
        }
    }

    //LD I, addr
    fn op_annn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.index_register = self.nnn;
    }

    //JP V0, addr
    fn op_bnnn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.program_counter = (self.nnn + data.variable_register[0] as u16) as usize;
    }

    //RND Vx, byte
    fn op_cxkk(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut rng = rand::thread_rng();
        data.variable_register[self.x] = rng.gen_range(0..0xFF + 1) as u8 & self.kk;
    }

    //DRW Vx, Vy, nibble
    fn op_dxyn(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut x_coordinate: usize;
        let mut y_coordinate: usize;
        let mut sprite: u8;

        data.variable_register[CARRY_FLAG] = BitState::UNSET;
        for row in 0..self.n as usize {
            y_coordinate = (data.variable_register[self.y] as usize + row) % self.max_rows;
            sprite = data.memory[data.index_register as usize + row];
            for column in 0..SPRITE_WIDTH {
                x_coordinate =
                    (data.variable_register[self.x] as usize + column) % self.max_columns;
                if (sprite & (0x80 >> column)) != BitState::UNSET {
                    if data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate]
                        == BitState::SET
                    {
                        data.variable_register[CARRY_FLAG] = BitState::SET;
                    }
                    data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate] ^=
                        BitState::SET;
                }
            }
        }
    }

    //DRW 16x16
    fn op_dxy0(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut x_coordinate: usize;
        let mut y_coordinate: usize;
        let mut sprite: u16;

        data.variable_register[CARRY_FLAG] = BitState::UNSET;
        for row in (0..BIG_SPRITE * 2).step_by(2) {
            y_coordinate = (data.variable_register[self.y] as usize + row) % self.max_rows;
            sprite = (data.memory[data.index_register as usize + row] as u16) << 8
                | data.memory[data.index_register as usize + row + 1] as u16;

            for column in 0..BIG_SPRITE {
                x_coordinate =
                    (data.variable_register[self.x] as usize + column) % self.max_columns;
                if (sprite & (0x8000 >> column)) != BitState::UNSET as u16 {
                    if data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate]
                        == BitState::SET
                    {
                        data.variable_register[CARRY_FLAG] = BitState::SET;
                    }
                    data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate] ^=
                        BitState::SET;
                }
            }
        }
    }

    //SKP Vx
    fn op_ex9e(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut keypad_borrow = self.keypad.lock().unwrap();
        if keypad_borrow.get_key(data.variable_register[self.x]) == KeyPress::Down as u8 {
            data.program_counter += PROGRAM_STEP;
            keypad_borrow.reset_key(data.variable_register[self.x]);
        }
    }

    //SKNP Vx
    fn op_exa1(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut keypad_borrow = self.keypad.lock().unwrap();
        if keypad_borrow.get_key(data.variable_register[self.x]) == KeyPress::Up as u8 {
            data.program_counter += PROGRAM_STEP;
        }
        keypad_borrow.reset_key(data.variable_register[self.x]);
    }

    //LD Vx, DT
    fn op_fx07(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.variable_register[self.x] = data.delay_timer;
    }

    //LD Vx, K
    fn op_fx0a(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let mut keypad_borrow = self.keypad.lock().unwrap();
        if let Some(key) = keypad_borrow.get_pressed_key() {
            data.variable_register[self.x] = key;
            keypad_borrow.reset_key(key);
        } else {
            data.program_counter -= PROGRAM_STEP;
        }
    }

    //LD DT, Vx
    fn op_fx15(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.delay_timer = data.variable_register[self.x];
    }

    //LD ST, Vx
    fn op_fx18(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.sound_timer = data.variable_register[self.x];
    }

    //LD ADD I, Vx
    fn op_fx1e(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.index_register += data.variable_register[self.x] as u16;
    }

    //LD F, Vx
    fn op_fx29(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.index_register = (data.variable_register[self.x] as u16) * 0x5;
    }

    //LD SF, Vx
    fn op_fx30(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        data.index_register =
            (data.variable_register[self.x] as u16) * 0xA + FONTSET_HIGH_START as u16;
    }

    //LD B, Vx
    fn op_fx33(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let index = data.index_register as usize;
        data.memory[index] = data.variable_register[self.x] / 100;
        data.memory[index + 1] = (data.variable_register[self.x] / 10) % 10;
        data.memory[index + 2] = data.variable_register[self.x] % 10;
    }

    //LD [I], Vx
    fn op_fx55(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let index = data.index_register as usize;
        for i in 0..self.x + 1 {
            data.memory[index + i] = data.variable_register[i];
        }
    }

    //LD Vx, [I]
    fn op_fx65(&mut self) {
        let mut data = self.data_ref.lock().unwrap();
        let index = data.index_register as usize;
        if index + self.x < MEMORYSIZE {
            for i in 0..self.x + 1 {
                data.variable_register[i] = data.memory[index + i];
            }
        }
    }

    //LD Vx, FLAG
    fn op_fx75(&mut self) {
        println!("into flag");

        let mut data = self.data_ref.lock().unwrap();
        if self.x < FLAG_REGISTER_SIZE {
            for i in 0..self.x + 1 {
                data.flag_register[i] = data.variable_register[i];
            }
        }
    }

    //LD FLAG, Vx
    fn op_fx85(&mut self) {
        println!("out of flag");

        let mut data = self.data_ref.lock().unwrap();
        if self.x < FLAG_REGISTER_SIZE {
            for i in 0..self.x + 1 {
                data.variable_register[i] = data.flag_register[i];
            }
        }
    }


    fn draw_sprite(&mut self, sprite_size: u8) {
        let mut data = self.data_ref.lock().unwrap();
        let mut x_coordinate: usize;
        let mut y_coordinate: usize;
        let  height = BIG_SPRITE;
        let width = SPRITE_WIDTH;
        let bitmask: u16 = 0x80;
        let mut sprite: u16;

        data.variable_register[CARRY_FLAG] = BitState::UNSET;
        for row in 0..height as usize {
            y_coordinate = (data.variable_register[self.y] as usize + row) % self.max_rows;
            sprite = data.memory[data.index_register as usize + row] as u16;
            for column in 0..width {
                x_coordinate =
                    (data.variable_register[self.x] as usize + column) % self.max_columns;
                if (sprite & (bitmask >> column)) != BitState::UNSET as u16 {
                    if data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate]
                        == BitState::SET
                    {
                        data.variable_register[CARRY_FLAG] = BitState::SET;
                    }
                    data.graphic_array[(y_coordinate * self.max_columns) + x_coordinate] ^=
                        BitState::SET;
                }
            }
        }
    }
}
