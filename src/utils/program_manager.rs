
#[derive(Copy, Clone)]
pub enum ProgramState {
    Running,
    Stopped,
    Restart,
    NewProgram,
    Quit,
    Idle,
}
pub struct ProgramManager {
    current_state: ProgramState,
}

impl ProgramManager {
    pub fn new() -> ProgramManager {
        ProgramManager {
            current_state: ProgramState::Running,
        }
    }

    pub fn set_state(&mut self, state: ProgramState) {
        self.current_state = state;
    }

    pub fn get_state(&mut self) -> ProgramState {
        self.current_state
    }
}