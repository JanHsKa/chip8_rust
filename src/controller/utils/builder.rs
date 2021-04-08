use crate::controller::{Emulator, FileManager, ProgramManager};
use crate::model::{Cpu, Keypad, Memory, MemoryAccess};
use crate::sdl2::Sdl;
use crate::view::{
    DisplayManager, GameDisplay, InfoDisplay, InputChecker, MemoryDisplay, OpcodeDisplay,
    SoundManager, StackDisplay, View,
};
use std::cell::RefCell;
use std::{
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct Builder {
    //data: Memory
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {}
    }

    pub fn build_emulator(
        &mut self,
        new_keypad: Arc<Mutex<Keypad>>,
        file_path: String,
        data: Memory,
    ) -> Emulator {
        let data_ref = self.package_arc_mutex(data);
        let file_manager = FileManager::new(file_path);
        let access = self.package_arc_mutex(MemoryAccess::new(Arc::clone(&data_ref)));
        let program_manager =
            self.package_arc_mutex(ProgramManager::new(file_manager, Arc::clone(&access)));
        let cpu = Cpu::new(Arc::clone(&new_keypad), Arc::clone(&data_ref));
        let (audio_sender, audio_receiver) = channel();

        let view = View::new(
            Arc::clone(&new_keypad),
            Arc::clone(&program_manager),
            Arc::clone(&access),
            audio_receiver,
        );

        Emulator::new(cpu, Arc::clone(&program_manager), view, audio_sender)
    }

    pub fn build_displays(
        &mut self,
        display_manager: &mut DisplayManager,
        mem_access: &Arc<Mutex<MemoryAccess>>,
        prog_manager: &Arc<Mutex<ProgramManager>>,
    ) {
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
