
use crate::utils::{FileManager};
use crate::processor::{memory_constants};
use sdl2::event::Event;
use self::memory_constants::{MAX_PROGRAM_SIZE};
use crate::sdl2::keyboard::Keycode;


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

pub struct ProgramManager {
    current_state: ProgramState,
    file_manager: FileManager,
}

impl ProgramManager {
    pub fn new(new_file_manager: FileManager) -> ProgramManager {
        ProgramManager {
            current_state: ProgramState::Running,
            file_manager: new_file_manager,
        }
    }

    pub fn initialize(&mut self) {
        self.load_file();
    }

    pub fn press_key(&mut self, key: Keycode) {
        match key { 
            Keycode::F1 => self.restart_program(),
            Keycode::F5 => self.stop_or_continue(),
            Keycode::F6 => {},
            Keycode::F7 => {},
            Keycode::F8 => {},
            Keycode::Plus => {},
            Keycode::Minus => {},
            _ => {}
        }
    }

    fn restart_program(&mut self) {
        self.current_state = ProgramState::Restart;
    }

    fn stop_or_continue(&mut self) {
        if self.current_state == ProgramState::Running {
            self.current_state = ProgramState::Stopped;
        } else {
            self.current_state = ProgramState::Running;
        }
    }
    
    fn load_file(&mut self) {
        if self.file_manager.load_file().is_ok() {
            self.current_state = ProgramState::Running;
        } else {
            self.current_state = ProgramState::Idle;
        }
    }

    pub fn new_file(&mut self, file_name: &String) {
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

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        self.file_manager.get_file_content()
    }
}