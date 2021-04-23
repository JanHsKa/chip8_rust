use crate::controller::{DebugManager, ProgramManager};
use crate::defines::{KeyPress, ProgramState};
use crate::model::Keypad;
use crate::sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump, Sdl};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub struct InputChecker {
    event_pump: EventPump,
    keypad: Arc<Mutex<Keypad>>,
    program_manager: Arc<Mutex<ProgramManager>>,
    debug_manager: Arc<Mutex<DebugManager>>,
    program_keys: HashSet<Keycode>,
    //keypad_keys: HashSet<Keycode>,
}

impl InputChecker {
    pub fn new(
        sdl_context: Arc<Sdl>,
        new_keypad: Arc<Mutex<Keypad>>,
        new_program_manager: Arc<Mutex<ProgramManager>>,
        new_debug_manager: Arc<Mutex<DebugManager>>,
    ) -> InputChecker {
        let new_program_keys: HashSet<Keycode> = vec![
            Keycode::F1,
            Keycode::F2,
            Keycode::F4,
            Keycode::F5,
            Keycode::F6,
            Keycode::F7,
            Keycode::F8,
            Keycode::Plus,
            Keycode::Minus,
        ]
        .into_iter()
        .collect();

        let _new_keypad_keys: HashSet<Keycode> = vec![].into_iter().collect();

        InputChecker {
            event_pump: sdl_context.event_pump().unwrap(),
            keypad: new_keypad,
            program_manager: new_program_manager,
            debug_manager: new_debug_manager,
            program_keys: new_program_keys,
            //keypad_keys:
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
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => self.process_mouse_click(mouse_btn, x, y),
                Event::Quit { .. } => self.program_manager.lock().unwrap().quit(),
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
            | Keycode::Plus
            | Keycode::Minus
            | Keycode::L => self.program_manager.lock().unwrap().press_key(key),
            Keycode::F6 | Keycode::F3 | Keycode::F7 | Keycode::F8 => {
                self.debug_manager.lock().unwrap().press_key(key)
            }
            Keycode::F5 => {
                self.program_manager.lock().unwrap().press_key(key);
            }
            Keycode::K => {
                keypad_ref.toggle_key_reset();
            }

            _ => keypad_ref.press_key(key, KeyPress::Down),
        }
    }

    fn process_keyup(&mut self, key: Keycode) {
        let mut keypad_ref = self.keypad.lock().unwrap();
        match key {
            _ => keypad_ref.press_key(key, KeyPress::Up),
        }
    }

    fn process_mouse_click(&mut self, button: &MouseButton, x: &i32, y: &i32) {
        if *button == MouseButton::Left {
            self.debug_manager
                .lock()
                .unwrap()
                .set_breakpoint_on_mouse_click(x, y);
        }
    }
}
