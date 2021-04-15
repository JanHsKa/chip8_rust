use crate::controller::{FileManager, StateManager};
use crate::defines::ProgramState;
use crate::model::{GameProperties, MemoryAccess};

use sdl2::keyboard::Keycode;
use std::sync::{Arc, Mutex};

pub const BASE_PROGRAM_SPEED: u64 = 10;
pub const MINIMUM_SPEED: u64 = 1;
pub const MAXIMUM_SPEED: u64 = 100;
pub const SPEED_STEP: u64 = 1;

pub struct ProgramManager {
    file_manager: FileManager,
    state_manager: Arc<Mutex<StateManager>>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    game_properties: Arc<Mutex<GameProperties>>,
}

impl ProgramManager {
    pub fn new(
        new_file_manager: FileManager,
        new_state_manager: Arc<Mutex<StateManager>>,
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_game_properties: Arc<Mutex<GameProperties>>,
    ) -> ProgramManager {
        ProgramManager {
            file_manager: new_file_manager,
            state_manager: new_state_manager,
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
            Keycode::F2 => self.open_editor(),
            Keycode::F4 => self.dump_memory(),
            Keycode::F5 => self.stop_or_continue(),
            Keycode::F8 => {}
            Keycode::Plus => self.increase_speed(),
            Keycode::Minus => self.decrease_speed(),
            _ => {}
        }
    }

    fn open_editor(&mut self) {
        self.file_manager.open_editor();
    }

    fn increase_speed(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if properties.game_speed < MAXIMUM_SPEED {
            properties.game_speed += SPEED_STEP;
        }
    }

    fn decrease_speed(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        if properties.game_speed > MINIMUM_SPEED {
            properties.game_speed -= SPEED_STEP;
        }
    }

    fn restart_program(&mut self) {
        self.state_manager
            .lock()
            .unwrap()
            .update_state(ProgramState::Restart);
    }

    pub fn stop_or_continue(&mut self) {
        self.state_manager.lock().unwrap().toggle_continue();
    }

    fn dump_memory(&mut self) {
        self.file_manager
            .dump_memory(self.memory_access.lock().unwrap().get_complete_memory());
    }

    fn load_file(&mut self) {
        if self.file_manager.load_file().is_ok() {
            self.state_manager
                .lock()
                .unwrap()
                .update_state(ProgramState::NewProgram);
            self.update_game_properties();
        } else {
            self.state_manager
                .lock()
                .unwrap()
                .update_state(ProgramState::Idle);
        }
    }

    pub fn get_speed(&mut self) -> u64 {
        self.game_properties.lock().unwrap().game_speed
    }

    pub fn new_file(&mut self, file_name: &str) {
        if self.file_manager.load_file_if_possible(file_name).is_ok() {
            self.state_manager
                .lock()
                .unwrap()
                .update_state(ProgramState::NewProgram);
            self.update_game_properties();
        }
    }

    fn update_game_properties(&mut self) {
        let mut properties = self.game_properties.lock().unwrap();
        properties.game_size = self.file_manager.get_file_info().file_size as usize;
        properties.game_name = self.file_manager.get_file_name();
        properties.game_code = self.file_manager.get_file_content();
    }

    pub fn quit(&mut self) {
        self.state_manager
            .lock()
            .unwrap()
            .update_state(ProgramState::Quit);
        println!("quit");
    }

    pub fn get_state(&mut self) -> ProgramState {
        self.state_manager.lock().unwrap().get_state()
    }

    pub fn get_file_content(&mut self) -> Vec<u8> {
        self.file_manager.get_file_content()
    }
}
