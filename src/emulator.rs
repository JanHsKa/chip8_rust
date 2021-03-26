use crate::cpu::Cpu;
use crate::filemanager::FileManager;
use crate::gamedisplay::GameDisplay;
use std::io;
use io::Result;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub struct Emulator {
    cpu: Cpu,
    file_manager: FileManager,
    game_display: GameDisplay,
}


impl Emulator {
    pub fn new(file_path: String) -> Self {
        Emulator {
             cpu: Cpu::new(),
             file_manager: FileManager::new(file_path),
             game_display: GameDisplay::new(),
        }
    }

    pub fn start_program(&mut self) {
        /* thread::spawn(|| {
            let mut game_display = GameDisplay::new();
            game_display.initialize();
        }); */
        //self.game_display.initialize();

        if self.file_manager.load_file().is_ok() {
            self.initialize();
            self.run_program();
        } else {
            println!("Error: Could not start program");
        }
    }

    fn run_program(&mut self) {
        self.initialize();
        let mut run = true;
        let mut timer = 0;

        while run {
            self.game_display.check_input();
            self.cpu.run_opcode();
            self.cpu.tick_timer();
            if timer == 60 {
                self.game_display.draw(&self.cpu.get_graphic_array());
                timer = 0;
            }
            println!("time: {}", timer);
            timer += 1;
            thread::sleep(Duration::from_millis(1));
        }

    }

    fn initialize(&mut self) {
        self.cpu.load_program_code(self.file_manager.get_file_content());
        println!("INIT");
    }
} 

