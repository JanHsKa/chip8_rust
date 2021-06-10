use crate::model::GameProperties;

use std::sync::{Arc, Mutex};

pub struct GamePropertiesAccess {
    game_properties: Arc<Mutex<GameProperties>>,
}

impl GamePropertiesAccess {
    pub fn new(new_properties: Arc<Mutex<GameProperties>>) -> GamePropertiesAccess {
        GamePropertiesAccess {
            game_properties: new_properties,
        }
    }

    pub fn get_game_name(&mut self) -> String {
        self.game_properties.lock().unwrap().game_name.clone()
    }

    pub fn get_game_size(&mut self) -> usize {
        self.game_properties.lock().unwrap().game_size
    }

    pub fn get_game_speed(&mut self) -> u64 {
        self.game_properties.lock().unwrap().game_speed
    }

    pub fn get_game_code(&mut self) -> Vec<u8> {
        self.game_properties.lock().unwrap().game_code.clone()
    }
}
