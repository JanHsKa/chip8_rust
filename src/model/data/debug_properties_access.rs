use crate::defines::{memory_constants::MAX_PROGRAM_SIZE, DebugState};
use crate::model::{DebugProperties, MemoryAccess};
use crate::model::{Memory, Resolution};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct DebugPropertiesAccess {
    debug_properties: Arc<Mutex<DebugProperties>>,
}

impl DebugPropertiesAccess {
    pub fn new(new_properties: Arc<Mutex<DebugProperties>>) -> DebugPropertiesAccess {
        DebugPropertiesAccess {
            debug_properties: new_properties,
        }
    }

    pub fn get_debug_state(&mut self) -> DebugState {
        self.debug_properties.lock().unwrap().debug_state
    }

    pub fn get_breakpoints(&mut self) -> HashMap<usize, u16> {
        self.debug_properties.lock().unwrap().breakpoints.clone()
    }
}
