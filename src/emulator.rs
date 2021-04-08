use crate::processor::{Cpu, MemoryAccess};
use crate::utils::{
    InputChecker, ProgramManager, BASE_PROGRAM_SPEED,
    ProgramState, SoundManager, TimeManager, TimeTo};
use crate::display::{DisplayManager};

use std::{any::TypeId, result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{TryIter, Sender, Receiver, channel}}};


//use std::sync::mpsc;
//use mpsc::{Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    cpu: Cpu,
    display_manager: DisplayManager,
    sound_manager: SoundManager,
    memory_access: Arc<Mutex<MemoryAccess>>,
    program_manager: Arc<Mutex<ProgramManager>>,
    current_state: ProgramState,
    receiver: Receiver<TimeTo>,
    speed: u64,
    instructioncounter: u64,
    input_checker: InputChecker,
    time_manager: TimeManager,
}


impl Emulator {
    pub fn new(display: DisplayManager, new_cpu: Cpu, 
        sound: SoundManager, new_access: Arc<Mutex<MemoryAccess>>, 
        new_program_manager: Arc<Mutex<ProgramManager>>, new_input_checker: InputChecker) -> Emulator {

        let (new_sender, new_receiver) = channel();
        
        /* std::thread::spawn(move || {
            let mut time_manager = TimeManager::new(new_sender);
            time_manager.start_clock();
        }); */

        /* std::thread::spawn(move || {
            let mut input_checker = new_input_checker;
            input_checker.check_input();
        }); */

        /* std::thread::spawn(move || {
            let mut display_manager = display;
            display_manager.initialize();
        }); */

        Emulator {
            cpu: new_cpu,
            display_manager: display,
            sound_manager: sound,
            memory_access: new_access,
            program_manager: new_program_manager,
            current_state: ProgramState::NewProgram,
            receiver: new_receiver,
            speed: BASE_PROGRAM_SPEED,
            instructioncounter: 0,
            input_checker: new_input_checker,
            time_manager: TimeManager::new(new_sender),
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
                ProgramState::Running => self.check_time(),
                ProgramState::Restart => self.new_program(),
                ProgramState::Stopped => self.check_display(),
                ProgramState::Idle => self.idle(),
                ProgramState::Quit => break 'running,
                _ => {}
            }
            self.update_state();

            //thread::sleep(Duration::from_nanos(1));
            //if self.instructioncounter > self.speed {
                //thread::sleep(Duration::from_nanos(1));
               // self.instructioncounter = 0;
            //}
        }
    }

    fn idle(&mut self) {
        //self.refresh_check();
        thread::sleep(Duration::from_millis(10));
    }

    fn run_code(&mut self) {
        self.cpu.run_opcode();
        self.instructioncounter += 1;
        println!("instructions: {}", self.instructioncounter);
        //self.refresh();
    }

    fn refresh(&mut self) {
        self.speed = self.program_manager.lock().unwrap().get_speed();
        self.time_manager.set_speed(self.speed as u128);
        self.refresh_cpu_timer();
        self.refresh_display();
        println!("update: ");
        self.instructioncounter += 0;

    }

    fn refresh_cpu_timer(&mut self) {
        self.cpu.tick_timer();
        self.sound_check();
    }

    fn receive_time_update(&mut self) -> Vec<TimeTo> {
        //let message: Vec<TimeTo> = self.receiver.try_iter().filter_map(|s| Some(s)).collect();
        let mut message: Vec<TimeTo> = Vec::new();
        for i in self.receiver.try_iter() {
            message.push(i);
        }

        return message;
    }

    fn check_time(&mut self) {
        match self.time_manager.check_time() {
            TimeTo::Update => self.refresh(),
            TimeTo::Process => self.run_code(),
            _ => {},
        };
    }
    /* fn check_time(&mut self) {
        self.receive_time_update().iter().map(|m| match m {
            TimeTo::Update => self.refresh(),
            TimeTo::Process => self.run_code(),
            _ => {},
        });
    } */

    /* fn check_display(&mut self) {
        self.receive_time_update().iter().map(|m| match m {
            TimeTo::Update => self.refresh_display(),
            _ => {},
        });
    } */

    fn check_display(&mut self) {
        if self.time_manager.check_time() == TimeTo::Update {
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
        //self.display_manager.initialize()?;
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

