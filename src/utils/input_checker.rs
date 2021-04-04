use crate::utils::{Keypad, ProgramManager};
use sdl2::EventPump;
use crate::sdl2::keyboard::Keycode;
use sdl2::event::Event;
use crate::{Rc, RefCell};

enum KeyPress {
    Up,
    Down,
}
pub struct InputChecker {
    event_pump: EventPump,
    keypad: Rc<RefCell<Keypad>>,
    program_manager: Rc<RefCell<ProgramManager>>,
}

impl InputChecker {
    pub fn new(new_pump: EventPump, new_keypad: Rc<RefCell<Keypad>>, new_program_manager: Rc<RefCell<ProgramManager>>) -> InputChecker {
        InputChecker {
            event_pump: new_pump,
            keypad: new_keypad,
            program_manager: new_program_manager,
        }
    }

    pub fn check_input(&mut self) {
        let mut events: Vec<Event> = Vec::new();
        for event in self.event_pump.poll_iter() {
            events.push(event);
        }

        for event in events.iter() {
            match event {
                Event::KeyDown {keycode,..} => self.process_keydown(keycode.unwrap()),
                Event::KeyUp {keycode,..} => self.process_keyup(keycode.unwrap()),
                //Event::Quit {..} => { self.quit = true },
                _ => {}
            }
        }
    }

    fn process_keydown(&mut self, key: Keycode) {
        let mut keypad_ref = self.keypad.borrow_mut();
        match key {
            _ => (*keypad_ref).press_key(key, KeyPress::Down as u8),

        }
    }

    fn process_keyup(&mut self, key: Keycode) {
        let mut keypad_ref = self.keypad.borrow_mut();
        match key {
            _ => (*keypad_ref).press_key(key, KeyPress::Up as u8),

        }
    }
}

