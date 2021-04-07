use crate::processor::{Cpu, MemoryAccess};
use crate::utils::{InputChecker, ProgramManager, ProgramState, SoundManager};
use crate::display::{DisplayManager};

use std::{result::Result};

use std::thread;
use std::time::Duration;

//use std::sync::mpsc;
//use mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    cpu: Cpu,
    display_manager: DisplayManager,
    sound_manager: SoundManager,
    memory_access: Rc<RefCell<MemoryAccess>>,
    input_checker: InputChecker,
    program_manager: Rc<RefCell<ProgramManager>>,
    current_state: ProgramState,
}


impl Emulator {
    pub fn new(display: DisplayManager, new_cpu: Cpu, 
        sound: SoundManager, new_access: Rc<RefCell<MemoryAccess>>, 
        new_input_checker: InputChecker, new_program_manager: Rc<RefCell<ProgramManager>>) -> Emulator {
        //let mut processor = Cpu::new(Rc::clone(&new_keypad), Memory::new());

        Emulator {
            cpu: new_cpu,
            display_manager: display,
            sound_manager: sound,
            memory_access: new_access,
            input_checker: new_input_checker,
            program_manager: new_program_manager,
            current_state: ProgramState::NewProgram,
        }
    }

    pub fn start_program(&mut self) {
        /* thread::spawn(|| {
            let mut game_display = GameDisplay::new();
            game_display.initialize();
        }); */

        self.initialize();
        self.update_state();
        self.run_program();
    }

    fn run_program(&mut self) {
        let mut timer = 0;

        'running: loop {
            timer += 1;
            self.input_checker.check_input();
            match self.current_state {
                ProgramState::NewProgram => self.new_program(),
                ProgramState::Running => self.run_code(&mut timer),
                ProgramState::Restart => self.new_program(),
                ProgramState::Stopped => self.refresh_display(&mut timer),
                ProgramState::Idle => {},
                ProgramState::Quit => break 'running,
                _ => {}
            }
            self.update_state();

            thread::sleep(Duration::from_nanos(self.program_manager.borrow_mut().get_speed()));
        }
        /* while self.current_state != ProgramState::Quit {
            timer += 1;
            //self.display_manager.check_input();
            self.input_checker.check_input();
            self.cpu.run_opcode();
            if timer == 16 {
                self.cpu.tick_timer();
                self.sound_check();
                self.display_manager.draw();
                timer = 0;
            }
            run = self.cpu.get_state() && 
            (self.program_manager.borrow_mut().get_state() != ProgramState::Quit);

            thread::sleep(Duration::from_millis(1));
        } */
    }

    fn run_code(&mut self, timer: &mut i32) {
        self.cpu.run_opcode();
        self.refresh(timer);
    }

    fn refresh(&mut self, timer: &mut i32) {
        self.refresh_cpu_timer(timer);
        self.refresh_display(timer);
    }

    fn refresh_cpu_timer(&mut self, timer: &mut i32) {
        if *timer == 16 {
            self.cpu.tick_timer();
            self.sound_check();
        }
    }

    fn refresh_display(&mut self, timer: &mut i32) {
        if *timer == 16 {
            self.display_manager.draw();
            *timer = 0;
        }
    }

    fn update_state(&mut self) {
        let mut manager = self.program_manager.borrow_mut();

        if manager.get_state() == ProgramState::Quit {
            self.current_state = ProgramState::Quit;
        } else if !self.cpu.get_state() {
            self.current_state = ProgramState::Idle;
        } else {
            self.current_state = manager.get_state();
        }
    }

    fn initialize(& mut self) -> Result<(), String> {
        let mut manager = self.program_manager.borrow_mut();
        manager.initialize();
        self.cpu.load_program_code(manager.get_file_content());
        self.display_manager.initialize()?;
        println!("INIT");

        Ok(())
    }

    fn new_program(&mut self) {
        let mut manager = self.program_manager.borrow_mut();
        self.cpu.reset();
        self.cpu.load_program_code(manager.get_file_content());
        manager.set_state(ProgramState::Running);
    }

    fn sound_check(&mut self) {
        if self.cpu.play_sound() {
            self.sound_manager.play_sound();
        } else {
            self.sound_manager.stop_sound();
        }
    }
} 

