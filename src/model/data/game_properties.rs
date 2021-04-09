use crate::defines::{ProgramState, game_constants::BASE_PROGRAM_SPEED};
use crate::controller::FileManager;

pub struct GameProperties {
        pub game_state: ProgramState,
        pub game_speed: u64,
        pub game_size: usize,
        pub game_name: String,
}

impl GameProperties {
    pub fn new() -> GameProperties {
        GameProperties {
            game_state: ProgramState::Running,
            game_speed: BASE_PROGRAM_SPEED,     
            game_size: 0,
            game_name: String::new(),  
        }
    }

    pub fn reset(&mut self) {
        *self = GameProperties::new();
    }
}