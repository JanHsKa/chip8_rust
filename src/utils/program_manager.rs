
use crate::utils::{FileManager, FileInfo};
use crate::processor::{memory_constants, MemoryAccess};
use sdl2::event::Event;
use self::memory_constants::{MAX_PROGRAM_SIZE};
use crate::sdl2::keyboard::Keycode;
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::render::TextureQuery;
use sdl2::ttf;


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
    memory_access: Rc<RefCell<MemoryAccess>>,
}

impl ProgramManager {
    pub fn new(new_file_manager: FileManager, 
        new_memory_access: Rc<RefCell<MemoryAccess>>) -> ProgramManager {

        ProgramManager {
            current_state: ProgramState::Running,
            file_manager: new_file_manager,
            memory_access: new_memory_access,
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
    
    fn dump_memory(&mut self) {
        self.file_manager.dump_memory(self.memory_access.borrow_mut().get_complete_memory());
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
            *iter = (file_content[offset + 2 * i] as u16) << 8 | file_content[offset + 2 * i + 1] as u16;
        } 

        Some(code_lines)
    } 
}