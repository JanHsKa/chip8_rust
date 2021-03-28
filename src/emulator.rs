use crate::cpu::Cpu;
use crate::filemanager::FileManager;
use crate::gamedisplay::GameDisplay;
use crate::keypad::Keypad;
use std::io;
use io::Result;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    cpu: Cpu,
    file_manager: FileManager,
    game_display: GameDisplay,
}


impl Emulator {
    pub fn new(file_path: String, new_keypad: Rc<RefCell<Keypad>>) -> Self {
        Emulator {
            cpu: Cpu::new(Rc::clone(&new_keypad)),
            file_manager: FileManager::new(file_path),
            game_display: GameDisplay::new(Rc::clone(&new_keypad)),
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
            //self.cpu.run_opcode();
            self.run_program();
        } else {
            println!("Error: Could not start program");
        }
    }

    fn run_program(&mut self) {
        let mut run = true;
        let mut timer = 0;

        while run {
            self.game_display.check_input();
            self.cpu.run_opcode();
            if timer == 16 {
                self.cpu.tick_timer();
                self.game_display.draw(self.cpu.get_graphic_array());
                timer = 0;
            }
            timer += 1;
            run = self.cpu.get_state();
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn initialize(&mut self) {
        self.cpu.load_program_code(self.file_manager.get_file_content());
        println!("INIT");
    }
} 

