#[derive(Copy, Clone, PartialEq)]
pub enum ProgramState {
    Game(GameState),
    Debug(DebugState),
    Running,
    Stopped,
    Restart,
    NewProgram,
    Quit,
    Idle,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Running,
    Stopped,
    Failed,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DebugState {
    Enabled,
    Disabled,
    Step,
    Breakpoint,
    Running,
    Stopped,
}

#[derive(Copy, Clone, PartialEq)]
pub enum CpuState {
    Running,
    Stopped,
}

#[derive(Copy, Clone, PartialEq)]
pub enum WindowState {
    Show,
    Hide,
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
