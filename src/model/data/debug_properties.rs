
use crate::defines::{DebugState};
use std::collections::HashMap;

pub struct DebugProperties {
    pub debug_state: DebugState,
    pub breakpoints: HashMap<usize, u16>,
}

impl Default for DebugProperties {
    fn default() -> Self {
        DebugProperties::new()
    }
}

impl DebugProperties {
    pub fn new() -> DebugProperties {
        DebugProperties {
            debug_state: DebugState::Disabled,
            breakpoints: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = DebugProperties::new();
    }
}
