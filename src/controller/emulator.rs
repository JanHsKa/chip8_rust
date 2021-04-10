use crate::controller::{ProgramManager, ProgramState, TimeManager, TimeTo, BASE_PROGRAM_SPEED};
use crate::model::{Cpu, MemoryAccess};
use crate::view::{DisplayManager, View};

use std::{
    any::TypeId,
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender, TryIter},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

//use std::sync::mpsc;
//use mpsc::{Sender, Receiver};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Emulator {
    cpu: Cpu,
    _view: View,
    program_manager: Arc<Mutex<ProgramManager>>,
    current_state: ProgramState,
    update_receiver: Receiver<TimeTo>,
    speed: u64,
    instructioncounter: u64,
    audio_sender: Sender<TimeTo>,
}

impl Emulator {
    pub fn new(
        new_cpu: Cpu,
        new_program_manager: Arc<Mutex<ProgramManager>>,
        new_view: View,
        new_audio_sender: Sender<TimeTo>,
    ) -> Emulator {
        let (new_sender, new_receiver) = channel();

        std::thread::spawn(move || {
            let mut time_manager = TimeManager::new(new_sender);
            time_manager.start_clock();
        });

        Emulator {
            cpu: new_cpu,
            _view: new_view,
            program_manager: new_program_manager,
            current_state: ProgramState::NewProgram,
            update_receiver: new_receiver,
            speed: BASE_PROGRAM_SPEED,
            instructioncounter: 0,
            audio_sender: new_audio_sender,
        }
    }

    pub fn start_program(&mut self) {
        self.initialize();
        self.update_state();
        self.run_program();
    }

    fn run_program(&mut self) {
        'running: loop {
            match self.current_state {
                ProgramState::NewProgram => self.new_program(),
                ProgramState::Running => self.check_time(),
                ProgramState::Step => self.step(),
                ProgramState::Restart => self.new_program(),
                ProgramState::Stopped => self.check_display(),
                ProgramState::Idle => self.idle(),
                ProgramState::Quit => break 'running,
                _ => {}
            }
            self.update_state();

            thread::sleep(Duration::from_micros(1000));
        }
    }

    fn idle(&mut self) {
        //self.refresh_check();
        thread::sleep(Duration::from_millis(10));
    }

    fn run_code(&mut self) {
        self.cpu.run_opcode();
        self.instructioncounter += 1;
    }

    fn refresh(&mut self) {
        self.speed = self.program_manager.lock().unwrap().get_speed();
        self.run_code_based_on_timer();
        self.refresh_cpu_timer();
        self.refresh_display();
        self.instructioncounter = 0;
    }

    fn run_code_based_on_timer(&mut self) {
        while self.instructioncounter < self.speed {
            self.run_code();
        }
    }

    fn refresh_cpu_timer(&mut self) {
        self.cpu.tick_timer();
        self.sound_check();
    }

    fn step(&mut self) {
        self.run_code();
        self.program_manager
            .lock()
            .unwrap()
            .set_state(ProgramState::Stopped);
    }

    fn check_time(&mut self) {
        if self.instructioncounter <= self.speed {
            self.run_code();
        }

        let msg = self.update_receiver.try_recv();
        if msg.is_ok() && msg.unwrap() == TimeTo::Update {
            self.refresh();
        }
    }

    fn check_display(&mut self) {
        let msg = self.update_receiver.try_recv();
        if msg.is_ok() && msg.unwrap() == TimeTo::Update {
            self.refresh_display();
        }
    }

    fn refresh_display(&mut self) {
        //self.display_manager.draw();
    }

    fn update_state(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        let state = manager.get_state();

        if state == ProgramState::Quit {
            self.current_state = ProgramState::Quit;
        } else if !self.cpu.get_state() {
            self.current_state = ProgramState::Idle;
        } else {
            self.current_state = manager.get_state();
        }
    }

    fn initialize(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        manager.initialize();
        self.cpu.load_program_code(manager.get_file_content());
    }

    fn new_program(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        self.cpu.reset();
        self.cpu.load_program_code(manager.get_file_content());
        manager.set_state(ProgramState::Running);
    }

    fn sound_check(&mut self) {
        if self.cpu.play_sound() {
            self.audio_sender.send(TimeTo::PlaySound).unwrap();
        } else {
            self.audio_sender.send(TimeTo::StopSound).unwrap();
        }
    }
}
