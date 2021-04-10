use crate::controller::FileManager;
use crate::defines::{game_constants::BASE_PROGRAM_SPEED, ProgramState};

pub struct GameProperties {
    pub game_state: ProgramState,
    pub game_speed: u64,
    pub game_size: usize,
    pub game_name: String,
}

impl Default for GameProperties {
    fn default() -> Self {
        GameProperties::new()
    }
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
