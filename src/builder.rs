use crate::processor::{Cpu, Memory, MemoryAccess};
use crate::display::{GameDisplay, DisplayManager, 
    InfoDisplay, StackDisplay, MemoryDisplay, OpcodeDisplay};
use crate::{Emulator};
use crate::utils::{FileManager, Keypad, InputChecker, ProgramManager, SoundManager};
use crate::sdl2::Sdl;
use std::cell::RefCell;
use std::{rc::Rc, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{
        Sender, Receiver, channel}}};

pub struct Builder {
    //data: Memory
}


impl Builder {
    pub fn new() -> Self {
        Builder {
        
        }
    }

    pub fn build_emulator(&mut self, new_keypad: Rc<RefCell<Keypad>>, 
        sdl_context: Sdl, file_path: String, data: Memory) -> Emulator{

        let data_ref = self.package_arc_mutex(data);
        let file_manager =  FileManager::new(file_path);
        let access = self.package_arc_mutex(MemoryAccess::new(Arc::clone(&data_ref)));
        let program_manager = self.package_arc_mutex(ProgramManager::new(file_manager, Arc::clone(&access)));
        let cpu = Cpu::new(Rc::clone(&new_keypad), Arc::clone(&data_ref));
        let sound_manager = SoundManager::new(&sdl_context);
        let input_checker = InputChecker::new(&sdl_context, 
            Rc::clone(&new_keypad), Arc::clone(&program_manager));
        let mut display_manager = DisplayManager::new(sdl_context);
        self.build_displays(&mut display_manager, &access, &program_manager);

        Emulator::new(display_manager, cpu, 
            sound_manager, Arc::clone(&access), input_checker, 
            Arc::clone(&program_manager))
    }

    fn build_displays(&mut self, display_manager: &mut DisplayManager, 
        mem_access: &Arc<Mutex<MemoryAccess>>,
        prog_manager: &Arc<Mutex<ProgramManager>>) {

        let game_display = GameDisplay::new(Arc::clone(&mem_access));
        let info_display = InfoDisplay::new(Arc::clone(&prog_manager));
        let stack_display = StackDisplay::new(Arc::clone(&mem_access));
        let memory_display = MemoryDisplay::new(Arc::clone(&mem_access));
        let opcode_display = OpcodeDisplay::new(Arc::clone(&mem_access), Arc::clone(&prog_manager));


        display_manager.add_display(Box::new(game_display));
        display_manager.add_display(Box::new(info_display));
        display_manager.add_display(Box::new(stack_display));
        display_manager.add_display(Box::new(memory_display));
        display_manager.add_display(Box::new(opcode_display));
    }

    fn package_rc_refcell<T>(&mut self, package: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(package))
    }

    fn package_arc_mutex<T>(&mut self, package: T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(package))
    }
}