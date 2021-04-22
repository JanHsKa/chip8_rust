use crate::controller::{
    DebugManager, ProgramManager, StateManager, TimeManager, TimeTo, BASE_PROGRAM_SPEED,
};
use crate::defines::{CpuState, DebugState, GameState, ProgramState};
use crate::model::Cpu;
use crate::view::View;

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

pub struct Emulator {
    cpu: Cpu,
    _view: View,
    program_manager: Arc<Mutex<ProgramManager>>,
    debug_manager: Arc<Mutex<DebugManager>>,
    state_manager: Arc<Mutex<StateManager>>,
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
        new_debug_manager: Arc<Mutex<DebugManager>>,
        new_state_manager: Arc<Mutex<StateManager>>,
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
            debug_manager: new_debug_manager,
            state_manager: new_state_manager,
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
            //println!("loop");
            let current_state = self.state_manager.lock().unwrap().get_state();
            match current_state {
                ProgramState::NewProgram => self.new_program(),
                ProgramState::Running => self.running(),
                ProgramState::Debug(DebugState::Step) => self.step(),
                ProgramState::Restart => self.new_program(),
                ProgramState::Stopped => self.check_debug(),
                ProgramState::Idle => self.idle(),
                ProgramState::Quit => break 'running,
                _ => {}
            }
            self.debug_manager.lock().unwrap().check_breakpoint();
            self.update_state();
            thread::sleep(Duration::from_micros(1000));
        }
    }

    fn check_debug(&mut self) {
        let _debug = self.debug_manager.lock().unwrap();
    }

    fn idle(&mut self) {
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
        self.refresh_cpu_timer();
    }

    fn running(&mut self) {
        self.run_remaining_opcodes();
        self.check_time();
    }

    fn run_remaining_opcodes(&mut self) {
        if self.instructioncounter <= self.speed {
            self.run_code();
        }
    }

    fn check_time(&mut self) {
        //let msg = self.update_receiver.try_recv();
        let mut is_ok = false;
        for _ in self.update_receiver.try_iter() {
            is_ok = true;
        }

        if is_ok {
            self.refresh();
        }
    }

    fn update_state(&mut self) {
        let mut state_manager = self.state_manager.lock().unwrap();

        let cpu_state = self.cpu.get_state();
        let state = state_manager.get_state();
        match (state, cpu_state) {
            (ProgramState::NewProgram, _) | (ProgramState::Restart, _) => {
                //println!("case: new program");
                state_manager.update_state(ProgramState::Running)
            }
            (ProgramState::Debug(DebugState::Step), CpuState::Running) => {
                state_manager.update_state(ProgramState::Stopped)
            }
            (_, CpuState::Stopped) => {
                //println!("stopped");
                state_manager.update_state(ProgramState::Game(GameState::Failed))
            }
            _ => {}
        }
    }

    fn initialize(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        manager.initialize();
        self.cpu.load_program_code(&manager.get_file_content());
    }

    fn new_program(&mut self) {
        //println!("new program");
        let mut manager = self.program_manager.lock().unwrap();
        self.cpu.reset();
        self.cpu.load_program_code(&manager.get_file_content());
    }

    fn sound_check(&mut self) {
        if self.cpu.play_sound() {
            self.audio_sender.send(TimeTo::PlaySound).unwrap();
        } else {
            self.audio_sender.send(TimeTo::StopSound).unwrap();
        }
    }
}
