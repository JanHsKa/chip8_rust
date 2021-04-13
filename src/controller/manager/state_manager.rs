use crate::defines::{DebugState, GameState, ProgramState};
use crate::model::States;
use std::sync::{Arc, Mutex};

pub struct StateManager {
    states: Arc<Mutex<States>>,
}

impl StateManager {
    pub fn new(new_states: Arc<Mutex<States>>) -> StateManager {
        StateManager { states: new_states }
    }

    pub fn set_state(&mut self, _state: ProgramState) {}

    pub fn toggle_continue(&mut self) {
        let mut state_data = self.states.lock().unwrap();
        let state = state_data.game_state;

        match state {
            GameState::Running => state_data.game_state = GameState::Stopped,
            GameState::Stopped => state_data.game_state = GameState::Running,
            _ => {}
        }
    }

    pub fn toggle_debug(&mut self) {
        let mut state_data = self.states.lock().unwrap();
        let debug_state = state_data.debug_state;
        let _program_state = state_data.program_state;
        let game_state = state_data.game_state;

        if (debug_state, game_state) == (DebugState::Disabled, GameState::Running) {
            state_data.debug_state = DebugState::Running;
            state_data.game_state = GameState::Stopped;
        }
    }

    pub fn update_state(&mut self, state: ProgramState) -> Option<ProgramState> {
        match state {
            ProgramState::NewProgram => self.new_program(),
            ProgramState::Game(GameState::Failed) => self.failed_game(),
            _ => {}
        }
        None
    }

    fn failed_game(&mut self) {}

    fn new_program(&mut self) {}
}
