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

    pub fn update_state(&mut self, state: ProgramState) {}
}
