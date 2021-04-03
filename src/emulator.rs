use crate::processor::{Cpu, Memory, MemoryAccess};
use crate::utils::FileManager;
use crate::display::{DisplayManager, GameDisplay};
use crate::keypad::Keypad;
use crate::sound_manager::SoundManager;
use crate::builder::Builder;

use crate::sdl2;
use crate::interfaces::Display;

use std::io;
use self::io::Result;
use std::thread;
use std::time::Duration;
//use std::sync::mpsc;
//use mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::Sdl;

pub struct Emulator {
    cpu: Cpu,
    file_manager: FileManager,
    display_manager: DisplayManager,
    sound_manager: SoundManager,
    memory_access: MemoryAccess,
}


impl Emulator {
    pub fn new(file: FileManager, display: DisplayManager, new_cpu: Cpu, sound: SoundManager, new_access: MemoryAccess) -> Emulator {
        //let mut processor = Cpu::new(Rc::clone(&new_keypad), Memory::new());

        Emulator {
            cpu: new_cpu,
            file_manager: file,
            display_manager: display,
            sound_manager: sound,
            memory_access: new_access,
        }
    }

    pub fn start_program(&mut self) {
        /* thread::spawn(|| {
            let mut game_display = GameDisplay::new();
            game_display.initialize();
        }); */

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
            timer += 1;
            self.display_manager.check_input();
            self.cpu.run_opcode();
            if timer == 16 {
                self.cpu.tick_timer();
                self.sound_check();
                self.display_manager.draw(self.cpu.get_graphic_array());
                timer = 0;
            }
            run = self.cpu.get_state() && !self.display_manager.get_quit();

            thread::sleep(Duration::from_millis(1));
        }
    }

    fn initialize(& mut self) {
        self.cpu.load_program_code(self.file_manager.get_file_content());
        self.display_manager.initialize();
        //let mut mem_access = self.cpu.get_memory_access();
        //self.display_manager.add_display(Box::new(GameDisplay::new(Rc::new(RefCell::new(mem_access)))));
        println!("INIT");
    }

    fn sound_check(&mut self) {
        if self.cpu.play_sound() {
            self.sound_manager.play_sound();
        } else {
            self.sound_manager.stop_sound();
        }
    }
} 

