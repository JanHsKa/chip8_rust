use crate::processor::{Cpu, Memory, MemoryAccess};
use crate::display::{GameDisplay, DisplayManager};
use crate::{Keypad, Emulator, sound_manager};
use crate::interfaces::Display;
use crate::utils::FileManager;
use sound_manager::SoundManager;
use crate::sdl2::Sdl;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Builder {
    //data: Memory
}


impl Builder {
    pub fn new() -> Self {
        Builder {
        
        }
    }

    pub fn build_game_display(&mut self, mem_access: MemoryAccess) {
        //GameDisplay::new(Rc::new(RefCell::new(*mem_access)))
    }

    pub fn build_emulator(&mut self, new_keypad: Rc<RefCell<Keypad>>, sdl_context: Sdl, file_path: String, data: Memory) -> Emulator{
        let data_ref = Rc::new(RefCell::new(data));
        let file_manager =  FileManager::new(file_path);
        let mut display_manager = DisplayManager::new(Rc::clone(&new_keypad), &sdl_context);
        let access = Rc::new(RefCell::new(MemoryAccess::new(Rc::clone(&data_ref))));
        display_manager.add_display(Box::new(GameDisplay::new(Rc::clone(&access))));
        let cpu = Cpu::new(Rc::clone(&new_keypad), Rc::clone(&data_ref));
        let sound_manager = SoundManager::new(&sdl_context);
        Emulator::new(file_manager, display_manager, cpu, sound_manager, Rc::clone(&access))
    }
}