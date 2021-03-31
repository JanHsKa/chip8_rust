mod emulator;
pub mod utils;
pub mod keypad;
pub mod input_checker;
pub mod sound_manager;
pub mod processor;
pub mod display;
pub mod interfaces;

extern crate sdl2;
use emulator::Emulator;
use std::env;
use std::thread;
use std::time::Duration;
use keypad::Keypad;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("start program");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let keypad = Rc::new(RefCell::new(Keypad::new()));
        let sdl_context = sdl2::init().unwrap();
        let mut emulator = Emulator::new(args[1].clone(), keypad, sdl_context);
        emulator.start_program();
    }
}
