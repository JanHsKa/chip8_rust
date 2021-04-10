#[derive(Copy, Clone, PartialEq)]
pub enum ProgramState {
    Running,
    Stopped,
    Restart,
    NewProgram,
    Quit,
    Idle,
    Step,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DebugState {
    Enabled,
    Disabled,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TimeTo {
    Update,
    Sleep,
    Process,
    PlaySound,
    StopSound,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Resolution {
    Low = 1,
    High = 2,
}

#[derive(Copy, Clone, PartialEq)]
pub struct BitState;

impl BitState {
    pub const UNSET: u8 = 0;
    pub const SET: u8 = 1;
}
