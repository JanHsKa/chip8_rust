use crate::model::DebugProperties;

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

    pub fn get_breakpoints(&mut self) -> HashMap<usize, u16> {
        self.debug_properties.lock().unwrap().breakpoints.clone()
    }
}
