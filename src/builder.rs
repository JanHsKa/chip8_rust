use crate::processor::{Cpu, Memory, MemoryAccess};
use crate::display::{GameDisplay, DisplayManager, InfoDisplay, StackDisplay};
use crate::{Emulator, sound_manager};
use crate::interfaces::IDisplay;
use crate::utils::{FileManager, Keypad, InputChecker, ProgramManager};
use sound_manager::SoundManager;
use crate::sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

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

    pub fn build_emulator(&mut self, new_keypad: Rc<RefCell<Keypad>>, 
        sdl_context: Sdl, file_path: String, data: Memory) -> Emulator{

        let data_ref = Rc::new(RefCell::new(data));
        let file_manager =  FileManager::new(file_path);
        let mut display_manager = DisplayManager::new(Rc::clone(&new_keypad), &sdl_context);
        let access = Rc::new(RefCell::new(MemoryAccess::new(Rc::clone(&data_ref))));
        let program_manager = Rc::new(RefCell::new(ProgramManager::new(file_manager, Rc::clone(&access))));
        self.build_displays(&mut display_manager, &access, &program_manager);
        let cpu = Cpu::new(Rc::clone(&new_keypad), Rc::clone(&data_ref));
        let sound_manager = SoundManager::new(&sdl_context);
        let input_checker = InputChecker::new(&sdl_context, 
            Rc::clone(&new_keypad), Rc::clone(&program_manager));

        Emulator::new(display_manager, cpu, 
            sound_manager, Rc::clone(&access), input_checker, 
            Rc::clone(&program_manager))
    }

    fn build_displays(&mut self, display_manager: &mut DisplayManager, 
        mem_access: &Rc<RefCell<MemoryAccess>>,
        prog_manager: &Rc<RefCell<ProgramManager>>) {
        let game_display = GameDisplay::new(Rc::clone(&mem_access));
        let info_display = InfoDisplay::new(Rc::clone(&prog_manager));
        let stack_display = StackDisplay::new(Rc::clone(&mem_access));

        display_manager.add_display(Box::new(game_display));
        display_manager.add_display(Box::new(info_display));
        display_manager.add_display(Box::new(stack_display));

    }
}