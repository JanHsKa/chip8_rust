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

    pub fn get_state(&mut self) -> ProgramState {
        self.states.lock().unwrap().program_state
    }

    pub fn get_game_state(&mut self) -> GameState {
        self.states.lock().unwrap().game_state
    }

    pub fn get_debug_state(&mut self) -> DebugState {
        self.states.lock().unwrap().debug_state
    }

    pub fn toggle_continue(&mut self) {
        let mut state_data = self.states.lock().unwrap();
        let state = state_data.game_state;

        match state {
            GameState::Running => {
                state_data.game_state = GameState::Stopped;
                state_data.program_state = ProgramState::Stopped;
            }
            GameState::Stopped => {
                state_data.game_state = GameState::Running;
                state_data.program_state = ProgramState::Running;
            }
            _ => {}
        }
    }

    pub fn toggle_debug(&mut self) {
        let mut state_data = self.states.lock().unwrap();
        let debug_state = state_data.debug_state;

        match debug_state {
            DebugState::Enabled => state_data.debug_state = DebugState::Disabled,
            DebugState::Disabled => state_data.debug_state = DebugState::Enabled,
            _ => {}
        }
    }

    pub fn update_state(&mut self, state: ProgramState) {
        match state {
            ProgramState::NewProgram => self.new_program(),
            ProgramState::Game(GameState::Failed) => self.failed_game(),
            ProgramState::Running => self.running(),
            ProgramState::Restart => self.restart(),
            ProgramState::Debug(DebugState::Breakpoint) => self.breakpoint(),
            ProgramState::Debug(DebugState::Step) => self.step(),
            ProgramState::Quit => self.quit(),
            ProgramState::Stopped => self.stop(),
            _ => {}
        }
    }

    fn stop(&mut self) {
        let mut states = self.states.lock().unwrap();
        states.program_state = ProgramState::Stopped;
        states.game_state = GameState::Stopped;
    }

    fn quit(&mut self) {
        let mut states = self.states.lock().unwrap();
        states.program_state = ProgramState::Quit;
        states.game_state = GameState::Stopped;
        states.debug_state = DebugState::Disabled;
    }

    fn breakpoint(&mut self) {
        let mut states = self.states.lock().unwrap();
        if states.game_state == GameState::Running && states.debug_state == DebugState::Enabled {
            states.program_state = ProgramState::Stopped;
            states.game_state = GameState::Stopped;
        }
    }

    fn step(&mut self) {
        let mut states = self.states.lock().unwrap();
        if states.game_state == GameState::Stopped && states.debug_state == DebugState::Enabled {
            states.program_state = ProgramState::Debug(DebugState::Step);
        }
    }

    fn failed_game(&mut self) {
        let mut states = self.states.lock().unwrap();
        states.debug_state = DebugState::Disabled;
        states.game_state = GameState::Failed;
        states.program_state = ProgramState::Idle;
    }

    fn new_program(&mut self) {
        let mut states = self.states.lock().unwrap();
        states.debug_state = DebugState::Disabled;
        states.game_state = GameState::Running;
        states.program_state = ProgramState::NewProgram;
    }

    fn running(&mut self) {
        let mut states = self.states.lock().unwrap();
        let game_state = states.game_state;
        let program_state = states.program_state;

        /* match (game_state, program_state) {
            (_, ProgramState::NewProgram) |
            (_, ProgramState::Restart) => {}
            (GameState::Running, _) => states.program_state = ProgramState::Running,
            (GameState::Running, _) => states.program_state = ProgramState::Running,

            _ => states.program_state = ProgramState::Stopped
        } */
        if states.game_state == GameState::Running {
            states.program_state = ProgramState::Running;
        } else {
            states.program_state = ProgramState::Stopped;
        }
    }

    fn restart(&mut self) {
        let mut states = self.states.lock().unwrap();
        states.game_state = GameState::Running;
        states.program_state = ProgramState::Restart;
    }
}
