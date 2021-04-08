use crate::display::layout_constants;
use crate::processor::{MemoryAccess, Resolution,
    memory_constants::{ROWS, COLUMNS, GRAPHIC_SIZE}};
use crate::interfaces::IDisplay;
use crate::utils::ProgramManager;
use std::{
    rc::Rc, cell::RefCell,
     result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, MutexGuard, mpsc::{
        Sender, Receiver, channel}}};

pub struct AccessPoint {
    memory_access: Arc<Mutex<MemoryAccess>>,
    program_manager: Arc<Mutex<ProgramManager>>,
    program_manager_mutex: Option<MutexGuard<ProgramManager>>,
    memory_access_mutex: Option<MutexGuard<MemoryAccess>>,
}

impl AccessPoint {
    pub fn new(new_memory_access: Arc<Mutex<MemoryAccess>>, new_program_manager: Arc<Mutex<ProgramManager>>) -> AccessPoint {
        AccessPoint {
            memory_access: new_memory_access,
            program_manager: new_program_manager,
            program_manager_mutex: None,
            memory_access_mutex: None,
        }
    }


    pub fn lock_all(&mut self) {
        self.program_manager_mutex = Some(self.program_manager.lock().unwrap());
        self.memory_access_mutex = Some(self.memory_access.lock().unwrap());
    }

    pub fn drop_all(&mut self) {
        self.memory_access_mutex = None;
        self.program_manager_mutex = None;
    }

    pub fn get_memory_access(&mut self) -> Option<MutexGuard<MemoryAccess>> {
        self.memory_access_mutex
    }

    pub fn get_program_manager(&mut self) -> Option<MutexGuard<ProgramManager>> {
        self.program_manager_mutex
    }
}