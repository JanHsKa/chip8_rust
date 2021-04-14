use crate::controller::StateManager;
use crate::defines::{
    font_constants::FONTSIZE_LINE,
    layout_constants::{OPCODE_HEIGHT, OPCODE_START_X, OPCODE_START_Y, OPCODE_WIDTH},
    memory_constants::{MAX_PROGRAM_SIZE, PROGRAM_START, VARIABLES_COUNT},
    DebugState, GameState, ProgramState,
};
use crate::model::{DebugProperties, MemoryAccess};
use sdl2::keyboard::Keycode;
use std::sync::{Arc, Mutex};

pub struct DebugManager {
    current_state: DebugState,
    debug_properties: Arc<Mutex<DebugProperties>>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    state_manager: Arc<Mutex<StateManager>>,
}

impl DebugManager {
    pub fn new(
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_state_manager: Arc<Mutex<StateManager>>,
        new_debug_properties: Arc<Mutex<DebugProperties>>,
    ) -> DebugManager {
        DebugManager {
            current_state: DebugState::Disabled,
            debug_properties: new_debug_properties,
            memory_access: new_memory_access,
            state_manager: new_state_manager,
        }
    }

    pub fn press_key(&mut self, key: Keycode) {
        match key {
            Keycode::F3 => self.toggle_enabled(),
            Keycode::F6 => self.step_trough(),
            Keycode::F7 => self.set_breakpoint_on_current_line(),
            Keycode::F8 => {}
            _ => {}
        }
    }

    fn toggle_enabled(&mut self) {
        self.state_manager.lock().unwrap().toggle_debug();
    }

    fn step_trough(&mut self) {
        self.state_manager
            .lock()
            .unwrap()
            .update_state(ProgramState::Debug(DebugState::Step));
    }

    fn set_breakpoint_on_current_line(&mut self) {
        let state = self.state_manager.lock().unwrap().get_debug_state();
        if state == DebugState::Enabled {
            let line = self.memory_access.lock().unwrap().get_program_counter() - PROGRAM_START;
            self.toggle_breakpoint(line);
        }
    }

    pub fn set_breakpoint_on_mouse_click(&mut self, x: &i32, y: &i32) {
        let mut state_manager = self.state_manager.lock().unwrap();
        if state_manager.get_debug_state() == DebugState::Enabled
            && state_manager.get_game_state() == GameState::Stopped
        {
            if let Some(line) = self.check_mouse_coordinates(x, y) {
                self.toggle_breakpoint(line as usize);
            }
        }
    }

    fn check_mouse_coordinates(&self, x: &i32, y: &i32) -> Option<i32> {
        if *x > OPCODE_START_X && *x < OPCODE_START_X + OPCODE_WIDTH as i32 {
            if *y > OPCODE_START_Y && *y < OPCODE_START_Y + OPCODE_HEIGHT as i32 {
                return Some((*y - OPCODE_START_Y) / FONTSIZE_LINE as i32);
            }
        }

        None
    }

    fn toggle_breakpoint(&self, line: usize) {
        let mut properties = self.debug_properties.lock().unwrap();
        if properties.breakpoints.contains_key(&line) {
            properties.breakpoints.remove(&line);
        } else if properties.breakpoints.len() < VARIABLES_COUNT {
            let opcode = self
                .memory_access
                .lock()
                .unwrap()
                .get_opcode_at(line)
                .unwrap();
            properties.breakpoints.insert(line, opcode);
        }
    }

    pub fn check_breakpoint(&mut self) {
        let mut memory_access = self.memory_access.lock().unwrap();
        let mut state_manager = self.state_manager.lock().unwrap();
        let breakpoints = self.debug_properties.lock().unwrap().breakpoints.clone();

        if let Some(_breakpoint) =
            breakpoints.get(&(memory_access.get_program_counter() - PROGRAM_START))
        {
            state_manager.update_state(ProgramState::Debug(DebugState::Breakpoint));
        }
    }

    pub fn toggle_debug_mode(&mut self) {}
}
