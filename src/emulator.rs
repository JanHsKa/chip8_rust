use crate::cpu::Cpu;
use crate::filemanager::FileManager;
use crate::GameDisplay::GameDisplay;
use std::io;
use io::Result;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub struct Emulator {
    cpu: Cpu,
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
        thread::spawn(|| {
            let mut game_display = GameDisplay::new();
            game_display.initialize();
        });
        //self.game_display.initialize();
        println!("program continue");
        /* if self.file_manager.load_file().is_ok() {
            self.run_program();
        } else {
            println!("Error: Could not start program");
        } */
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

