use crate::controller::{FileInfo, FileManager};
use crate::defines::memory_constants::{MAX_PROGRAM_SIZE, PROGRAM_START, VARIABLES_COUNT};
use crate::model::{GameProperties, MemoryAccess};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub const BASE_PROGRAM_SPEED: u64 = 10;

#[derive(Copy, Clone, PartialEq)]
pub enum ProgramState {
    Running,
    Stopped,
    Restart,
    NewProgram,
    Quit,
    Idle,
    Step,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DebugState {
    Enabled,
    Disabled,
}

pub struct ProgramManager {
    current_state: ProgramState,
    file_manager: FileManager,
    memory_access: Arc<Mutex<MemoryAccess>>,
    program_speed: u64,
    game_properties: Arc<Mutex<GameProperties>>,
    breakpoints: HashMap<usize, u16>,
}

impl ProgramManager {
    pub fn new(
        new_file_manager: FileManager,
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_game_properties: Arc<Mutex<GameProperties>>,
    ) -> ProgramManager {
        ProgramManager {
            current_state: ProgramState::Running,
            file_manager: new_file_manager,
            memory_access: new_memory_access,
            program_speed: BASE_PROGRAM_SPEED,
            game_properties: new_game_properties,
            breakpoints: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.load_file();
    }

    pub fn press_key(&mut self, key: Keycode) {
        match key {
            Keycode::F1 => self.restart_program(),
            Keycode::F4 => self.dump_memory(),
            Keycode::F5 => self.stop_or_continue(),
            Keycode::F6 => self.step_trough(),
            Keycode::F7 => self.set_breakpoint(),
            Keycode::F8 => {}
            Keycode::Plus => self.increase_speed(),
            Keycode::Minus => self.decrease_speed(),
            _ => {}
        }
    }

    fn step_trough(&mut self) {
        println!(
            "pc: {}",
            self.memory_access.lock().unwrap().get_program_counter() - PROGRAM_START
        );
        println!(
            "opcode: {:04X}",
            self.memory_access.lock().unwrap().get_opcode()
        );

        if self.current_state == ProgramState::Stopped {
            self.current_state = ProgramState::Step;
        }
    }

    fn set_breakpoint(&mut self) {
        if self.current_state == ProgramState::Stopped {
            let mut access = self.memory_access.lock().unwrap();
            let line = access.get_program_counter() - PROGRAM_START;
            if self.breakpoints.contains_key(&line) {
                self.breakpoints.remove(&line);
            } else if self.breakpoints.len() < VARIABLES_COUNT {
                let opcode = access.get_opcode();
                self.breakpoints.insert(line, opcode);
            }
        }
    }

    fn increase_speed(&mut self) {
        if self.program_speed < 100 {
            self.program_speed += 1;
        }
    }

    fn decrease_speed(&mut self) {
        if self.program_speed > 1 {
            self.program_speed -= 1;
        }
    }

    fn restart_program(&mut self) {
        self.current_state = ProgramState::Restart;
    }

    pub fn stop_or_continue(&mut self) {
        if self.current_state == ProgramState::Running {
            self.current_state = ProgramState::Stopped;
        } else {
            self.current_state = ProgramState::Running;
        }
    }

    fn dump_memory(&mut self) {
        self.file_manager
            .dump_memory(self.memory_access.lock().unwrap().get_complete_memory());
    }

    fn load_file(&mut self) {
        if self.file_manager.load_file().is_ok() {
            self.current_state = ProgramState::Running;
        } else {
            self.current_state = ProgramState::Idle;
        }
    }

    pub fn get_breakpoints(&mut self) -> HashMap<usize, u16> {
        self.breakpoints.clone()
    }

    pub fn get_speed(&mut self) -> u64 {
        self.program_speed
    }

    pub fn new_file(&mut self, file_name: &str) {
        if self.file_manager.load_file_if_possible(file_name).is_ok() {
            self.current_state = ProgramState::NewProgram;
        }
    }

    pub fn set_state(&mut self, state: ProgramState) {
        self.current_state = state;
    }

    pub fn get_state(&mut self) -> ProgramState {
        self.current_state
    }

    pub fn get_program_name(&mut self) -> String {
        self.file_manager.get_file_name()
    }

    pub fn get_file_info(&mut self) -> FileInfo {
        self.file_manager.get_file_info()
    }

    pub fn get_program_size(&mut self) -> usize {
        self.get_file_info().file_size as usize
    }

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        self.file_manager.get_file_content()
    }

    pub fn get_code_snippet(&mut self, count: usize, offset: usize) -> Option<Vec<u16>> {
        let program_size = self.get_program_size();
        if offset + count * 2 > program_size {
            return None;
        }

        let mut code_lines: Vec<u16> = vec![0; count];
        let file_content = &self.get_file_content();
        for (i, iter) in code_lines.iter_mut().enumerate() {
            *iter = (file_content[offset + 2 * i] as u16) << 8
                | file_content[offset + 2 * i + 1] as u16;
        }

        Some(code_lines)
    }
}
