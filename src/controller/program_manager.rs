use crate::controller::{FileInfo, FileManager};
use crate::defines::{
    memory_constants::{MAX_PROGRAM_SIZE, PROGRAM_START, VARIABLES_COUNT},
    ProgramState,
};
use crate::model::{GameProperties, MemoryAccess};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex},
};

pub const BASE_PROGRAM_SPEED: u64 = 10;

pub struct ProgramManager {
    file_manager: FileManager,
    memory_access: Arc<Mutex<MemoryAccess>>,
    game_properties: Arc<Mutex<GameProperties>>,
}

impl ProgramManager {
    pub fn new(
        new_file_manager: FileManager,
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_game_properties: Arc<Mutex<GameProperties>>,
    ) -> ProgramManager {
        ProgramManager {
            file_manager: new_file_manager,
            memory_access: new_memory_access,
            game_properties: new_game_properties,
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
            Keycode::F8 => {}
            Keycode::Plus => self.increase_speed(),
            Keycode::Minus => self.decrease_speed(),
            _ => {}
        }
    }

    fn increase_speed(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if properties.game_speed < 100 {
            properties.game_speed += 1;
        }
    }

    fn decrease_speed(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if properties.game_speed > 1 {
            properties.game_speed -= 1;
        }
    }

    fn restart_program(&mut self) {
        self.game_properties.lock().unwrap().game_state = ProgramState::Restart;
    }

    pub fn stop_or_continue(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if properties.game_state == ProgramState::Running {
            properties.game_state = ProgramState::Stopped;
        } else {
            properties.game_state = ProgramState::Running;
        }
    }

    fn dump_memory(&mut self) {
        self.file_manager
            .dump_memory(self.memory_access.lock().unwrap().get_complete_memory());
    }

    fn load_file(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if self.file_manager.load_file().is_ok() {
            properties.game_state = ProgramState::Running;
            properties.game_size = self.file_manager.get_file_info().file_size as usize;
        } else {
            properties.game_state = ProgramState::Idle;
        }
    }

    pub fn get_speed(&mut self) -> u64 {
        self.game_properties.lock().unwrap().game_speed
    }

    pub fn new_file(&mut self, file_name: &str) {
        if self.file_manager.load_file_if_possible(file_name).is_ok() {
            self.game_properties.lock().unwrap().game_state = ProgramState::NewProgram;
        }
    }

    pub fn set_state(&mut self, state: ProgramState) {
        self.game_properties.lock().unwrap().game_state = state;
    }

    pub fn get_state(&mut self) -> ProgramState {
        self.game_properties.lock().unwrap().game_state
    }

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        self.file_manager.get_file_content()
    }
}
