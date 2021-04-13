use crate::controller::{DebugManager, Emulator, FileManager, ProgramManager, StateManager};
use crate::model::{
    Cpu, DebugProperties, DebugPropertiesAccess, GameProperties, GamePropertiesAccess, Keypad,
    Memory, MemoryAccess, States, StatesAccess,
};

use crate::view::{
    BreakPointDisplay, DisplayManager, GameDisplay, InfoDisplay, InputChecker, MemoryDisplay,
    OpcodeDisplay, SoundManager, StackDisplay, View,
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

    pub fn build_emulator(&mut self, file_path: String) -> Emulator {
        let game_properties = self.package_arc_mutex(GameProperties::new());
        let game_properties_access =
            self.package_arc_mutex(GamePropertiesAccess::new(Arc::clone(&game_properties)));

        let debug_properties = self.package_arc_mutex(DebugProperties::new());
        let debug_properties_access =
            self.package_arc_mutex(DebugPropertiesAccess::new(Arc::clone(&debug_properties)));

        let states = self.package_arc_mutex(States::new());
        let _states_access = self.package_arc_mutex(StatesAccess::new(Arc::clone(&states)));
        let state_manager = self.package_arc_mutex(StateManager::new(Arc::clone(&states)));

        let data_ref = self.package_arc_mutex(Memory::new());
        let new_keypad = self.package_arc_mutex(Keypad::new());

        let file_manager = FileManager::new(file_path);
        let access = self.package_arc_mutex(MemoryAccess::new(Arc::clone(&data_ref)));
        let program_manager = self.package_arc_mutex(ProgramManager::new(
            file_manager,
            Arc::clone(&state_manager),
            Arc::clone(&access),
            Arc::clone(&game_properties),
        ));

        let debug_manager = self.package_arc_mutex(DebugManager::new(
            Arc::clone(&access),
            Arc::clone(&state_manager),
            Arc::clone(&debug_properties),
        ));

        let cpu = Cpu::new(Arc::clone(&new_keypad), Arc::clone(&data_ref));
        let (audio_sender, audio_receiver) = channel();

        let view = View::new(
            Arc::clone(&new_keypad),
            Arc::clone(&program_manager),
            Arc::clone(&debug_manager),
            Arc::clone(&game_properties_access),
            Arc::clone(&debug_properties_access),
            Arc::clone(&access),
            audio_receiver,
        );

        Emulator::new(
            cpu,
            Arc::clone(&program_manager),
            Arc::clone(&debug_manager),
            view,
            audio_sender,
        )
    }

    pub fn build_displays(
        &mut self,
        display_manager: &mut DisplayManager,
        mem_access: &Arc<Mutex<MemoryAccess>>,
        properties_access: &Arc<Mutex<GamePropertiesAccess>>,
        debug_properties_access: &Arc<Mutex<DebugPropertiesAccess>>,
    ) {
        let game_display = GameDisplay::new(Arc::clone(&mem_access));
        let info_display = InfoDisplay::new(Arc::clone(&properties_access));
        let stack_display = StackDisplay::new(Arc::clone(&mem_access));
        let memory_display = MemoryDisplay::new(Arc::clone(&mem_access));
        let opcode_display =
            OpcodeDisplay::new(Arc::clone(&mem_access), Arc::clone(&properties_access));
        let breakpoint_display = BreakPointDisplay::new(Arc::clone(debug_properties_access));

        display_manager.add_display(Box::new(game_display));
        display_manager.add_display(Box::new(info_display));
        display_manager.add_display(Box::new(stack_display));
        display_manager.add_display(Box::new(memory_display));
        display_manager.add_display(Box::new(opcode_display));
        display_manager.add_display(Box::new(breakpoint_display));
    }

    fn package_rc_refcell<T>(&mut self, package: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(package))
    }

    fn package_arc_mutex<T>(&mut self, package: T) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(package))
    }
}
