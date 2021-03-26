use crate::cpu::Cpu;
use crate::filemanager::FileManager;
use std::io;
use io::Result;

pub struct Emulator {
    cpu:    Cpu,
    file_manager: FileManager,
}


impl Emulator {
    pub fn new(file_path: String) -> Self {
        Emulator {
             cpu: Cpu::new(),
             file_manager: FileManager::new(file_path),
        }
    }

    pub fn start_program(&mut self) {
        if self.file_manager.load_file().is_ok() {
            self.run_program();
        } else {
            println!("Error: Could not start program");
        }
    }

    fn run_program(&mut self) {
        self.initialize();
        self.cpu.run_opcode();
    }

    fn initialize(&mut self) {
        self.cpu.load_program_code(self.file_manager.get_file_content());
        println!("INIT");
    }
} 

