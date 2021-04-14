use crate::defines::{DebugState, GameState, ProgramState};

pub struct States {
    pub program_state: ProgramState,
    pub debug_state: DebugState,
    pub game_state: GameState,
}

impl Default for States {
    fn default() -> Self {
        States::new()
    }
}

impl States {
    pub fn new() -> States {
        States {
            program_state: ProgramState::NewProgram,
            debug_state: DebugState::Disabled,
            game_state: GameState::Running,
        }
    }

    pub fn reset(&mut self) {
        *self = States::new();
    }
}
