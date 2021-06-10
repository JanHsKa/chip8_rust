use crate::model::{DebugPropertiesAccess, GamePropertiesAccess, MemoryAccess, StatesAccess};
use std::{
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread,
    time::Duration,
};

pub struct AccessPoint {
    memory_access: Arc<Mutex<MemoryAccess>>,
    game_properties_access: Arc<Mutex<GamePropertiesAccess>>,
    debug_properties_access: Arc<Mutex<DebugPropertiesAccess>>,
    states_access: Arc<Mutex<StatesAccess>>,
}

impl AccessPoint {
    pub fn new(
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_game_properties_access: Arc<Mutex<GamePropertiesAccess>>,
        new_debug_properties_access: Arc<Mutex<DebugPropertiesAccess>>,
        new_states_access: Arc<Mutex<StatesAccess>>,
    ) -> AccessPoint {
        AccessPoint {
            memory_access: new_memory_access,
            game_properties_access: new_game_properties_access,
            debug_properties_access: new_debug_properties_access,
            states_access: new_states_access,
        }
    }
}
