use crate::controller::{ProgramManager, ProgramState};
use crate::model::Keypad;
use crate::sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::Sdl;
use std::sync::{Arc, Mutex};

enum KeyPress {
    Up,
    Down,
}

pub struct InputChecker {
    event_pump: EventPump,
    keypad: Arc<Mutex<Keypad>>,
    program_manager: Arc<Mutex<ProgramManager>>,
}

unsafe impl Send for InputChecker {}

impl InputChecker {
    pub fn new(
        sdl_context: Arc<Sdl>,
        new_keypad: Arc<Mutex<Keypad>>,
        new_program_manager: Arc<Mutex<ProgramManager>>,
    ) -> InputChecker {
        InputChecker {
            event_pump: sdl_context.event_pump().unwrap(),
            keypad: new_keypad,
            program_manager: new_program_manager,
        }
    }

    pub fn check_input(&mut self) {
        let mut events: Vec<Event>;
        events = Vec::new();
        for event in self.event_pump.poll_iter() {
            events.push(event);
        }

        for event in events.iter() {
            match event {
                Event::KeyDown { keycode, .. } => self.process_keydown(keycode.unwrap()),
                Event::KeyUp { keycode, .. } => self.process_keyup(keycode.unwrap()),
                Event::DropFile { filename, .. } => {
                    self.program_manager.lock().unwrap().new_file(filename)
                }
                Event::Quit { .. } => self
                    .program_manager
                    .lock()
                    .unwrap()
                    .set_state(ProgramState::Quit),
                _ => {}
            }
        }
    }

    fn process_keydown(&mut self, key: Keycode) {
        let mut keypad_ref = self.keypad.lock().unwrap();
        match key {
            Keycode::F1
            | Keycode::F2
            | Keycode::F4
            | Keycode::F5
            | Keycode::F6
            | Keycode::F7
            | Keycode::F8
            | Keycode::Plus
            | Keycode::Minus => self.program_manager.lock().unwrap().press_key(key),
            _ => (*keypad_ref).press_key(key, KeyPress::Down as u8),
        }
    }

    fn process_keyup(&mut self, key: Keycode) {
        let mut keypad_ref = self.keypad.lock().unwrap();
        match key {
            _ => (*keypad_ref).press_key(key, KeyPress::Up as u8),
        }
    }
}
