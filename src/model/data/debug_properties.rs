use std::collections::HashMap;

pub struct DebugProperties {
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
            breakpoints: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = DebugProperties::new();
    }
}
