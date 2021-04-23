use crate::defines::{CpuState, DebugState, GameState, ProgramState};
use crate::model::States;
use std::sync::{Arc, Mutex};

pub struct StatesAccess {
    states: Arc<Mutex<States>>,
}

impl StatesAccess {
    pub fn new(new_states: Arc<Mutex<States>>) -> StatesAccess {
        StatesAccess { states: new_states }
    }

    pub fn get_program_state(&mut self) -> ProgramState {
        self.states.lock().unwrap().program_state
    }

    pub fn get_debug_state(&mut self) -> DebugState {
        self.states.lock().unwrap().debug_state
    }

    pub fn get_game_state(&mut self) -> GameState {
        self.states.lock().unwrap().game_state
    }

    pub fn get_cpu_state(&mut self) -> CpuState {
        self.states.lock().unwrap().cpu_state
    }
}
