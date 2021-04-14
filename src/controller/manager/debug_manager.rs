use crate::controller::StateManager;
use crate::defines::{
    memory_constants::{MAX_PROGRAM_SIZE, PROGRAM_START, VARIABLES_COUNT},
    DebugState, ProgramState,
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
            Keycode::F7 => self.set_breakpoint(),
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

    fn set_breakpoint(&mut self) {
        let mut properties = self.debug_properties.lock().unwrap();
        let mut state_manager = self.state_manager.lock().unwrap();
        if state_manager.get_debug_state() == DebugState::Enabled {
            let mut access = self.memory_access.lock().unwrap();
            let line = access.get_program_counter() - PROGRAM_START;
            if properties.breakpoints.contains_key(&line) {
                properties.breakpoints.remove(&line);
            } else if properties.breakpoints.len() < VARIABLES_COUNT {
                let opcode = access.get_opcode_at(line).unwrap();
                properties.breakpoints.insert(line, opcode);
            }
        }
    }

    pub fn check_breakpoint(&mut self) {
        let mut memory_access = self.memory_access.lock().unwrap();
        let mut state_manager = self.state_manager.lock().unwrap();
        let breakpoints = self.debug_properties.lock().unwrap().breakpoints.clone();

        if let Some(breakpoint) =
            breakpoints.get(&(memory_access.get_program_counter() - PROGRAM_START))
        {
            state_manager.update_state(ProgramState::Debug(DebugState::Breakpoint));
        }
    }

    pub fn toggle_debug_mode(&mut self) {}
}
