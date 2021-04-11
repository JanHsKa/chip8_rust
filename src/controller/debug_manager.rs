use crate::defines::DebugState;
use crate::defines::{
    memory_constants::{MAX_PROGRAM_SIZE, PROGRAM_START, VARIABLES_COUNT},
    ProgramState,
};
use crate::model::{DebugProperties, MemoryAccess};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct DebugManager {
    current_state: DebugState,
    debug_properties: Arc<Mutex<DebugProperties>>,
    memory_access: Arc<Mutex<MemoryAccess>>,
}

impl DebugManager {
    pub fn new(
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_debug_properties: Arc<Mutex<DebugProperties>>,
    ) -> DebugManager {
        DebugManager {
            current_state: DebugState::Disabled,
            debug_properties: new_debug_properties,
            memory_access: new_memory_access,
        }
    }

    pub fn press_key(&mut self, key: Keycode) {
        match key {
            Keycode::F5 => self.stop_or_continue(),
            Keycode::F6 => self.step_trough(),
            Keycode::F7 => self.set_breakpoint(),
            Keycode::F8 => {}
            _ => {}
        }
    }

    pub fn stop_or_continue(&mut self) {
        let mut properties = self.debug_properties.lock().unwrap();
        println!("stop");
        if properties.debug_state == DebugState::Running {
            properties.debug_state = DebugState::Stopped;
        } else {
            properties.debug_state = DebugState::Running;
        }
    }

    fn step_trough(&mut self) {
        let mut properties = self.debug_properties.lock().unwrap();
        if properties.debug_state == DebugState::Stopped {
            properties.debug_state = DebugState::Step;
        }
    }

    fn set_breakpoint(&mut self) {
        let mut properties = self.debug_properties.lock().unwrap();
        if properties.debug_state == DebugState::Stopped {
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

    pub fn toggle_debug_mode(&mut self) {}
}
