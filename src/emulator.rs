use crate::processor::{Cpu, MemoryAccess};
use crate::utils::{
    InputChecker, ProgramManager, BASE_PROGRAM_SPEED,
    ProgramState, SoundManager, TimeManager, TimeTo};
use crate::display::{DisplayManager};

use std::{result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}}};


//use std::sync::mpsc;
//use mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    cpu: Cpu,
    display_manager: DisplayManager,
    sound_manager: SoundManager,
    memory_access: Arc<Mutex<MemoryAccess>>,
    input_checker: InputChecker,
    program_manager: Arc<Mutex<ProgramManager>>,
    current_state: ProgramState,
    receiver: Receiver<TimeTo>,
    speed: u64
}


impl Emulator {
    pub fn new(display: DisplayManager, new_cpu: Cpu, 
        sound: SoundManager, new_access: Arc<Mutex<MemoryAccess>>, 
        new_input_checker: InputChecker, new_program_manager: Arc<Mutex<ProgramManager>>) -> Emulator {
        //let mut processor = Cpu::new(Rc::clone(&new_keypad), Memory::new());
        let (new_sender, new_receiver) = channel();
        
        std::thread::spawn(move || {
            let mut time_manager = TimeManager::new(new_sender);
            time_manager.start_clock();
        });

        Emulator {
            cpu: new_cpu,
            display_manager: display,
            sound_manager: sound,
            memory_access: new_access,
            input_checker: new_input_checker,
            program_manager: new_program_manager,
            current_state: ProgramState::NewProgram,
            receiver: new_receiver,
            speed: BASE_PROGRAM_SPEED,
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
        'running: loop {
            self.input_checker.check_input();
            match self.current_state {
                ProgramState::NewProgram => self.new_program(),
                ProgramState::Running => self.run_code(),
                ProgramState::Restart => self.new_program(),
                ProgramState::Stopped => self.refresh_only_display(),
                ProgramState::Idle => {},
                ProgramState::Quit => break 'running,
                _ => {}
            }
            self.update_state();

            thread::sleep(Duration::from_nanos(self.speed));
        }
    }

    fn run_code(&mut self) {
        self.cpu.run_opcode();
        self.refresh();
    }

    fn refresh(&mut self) {
        if self.refresh_check() {
            self.speed = self.program_manager.lock().unwrap().get_speed();
            self.refresh_cpu_timer();
            self.refresh_display();
        }
    }

    fn refresh_cpu_timer(&mut self) {
        self.cpu.tick_timer();
        self.sound_check();
    }

    fn refresh_check(&mut self) -> bool {
        let message = self.receiver.try_recv();
        if message.is_ok() {
            return message.unwrap() == TimeTo::Update
        }

        return false;
    }
    fn refresh_only_display(&mut self) {
        if self.refresh_check() {
            self.refresh_display();
        }
    }

    fn refresh_display(&mut self) {
        self.display_manager.draw();
    }

    fn update_state(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();

        if manager.get_state() == ProgramState::Quit {
            self.current_state = ProgramState::Quit;
        } else if !self.cpu.get_state() {
            self.current_state = ProgramState::Idle;
        } else {
            self.current_state = manager.get_state();
        }
    }

    fn initialize(& mut self) -> Result<(), String> {
        let mut manager = self.program_manager.lock().unwrap();
        manager.initialize();
        self.cpu.load_program_code(manager.get_file_content());
        self.display_manager.initialize()?;
        println!("INIT");

        Ok(())
    }

    fn new_program(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
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

