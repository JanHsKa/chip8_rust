pub mod builder;
pub mod display;
mod emulator;
pub mod interfaces;
pub mod processor;
pub mod utils;

extern crate edit;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use crate::builder::Builder;
use crate::processor::Memory;
use emulator::Emulator;
use std::env;
use std::sync::{Arc, Mutex};
use utils::Keypad;

fn main() {
    println!("start program");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let keypad = Arc::new(Mutex::new(Keypad::new()));
        let memory = Memory::new();
        let mut builder = Builder::new();
        let mut emulator = builder.build_emulator(keypad, args[1].clone(), memory);
        //let mut emulator = Emulator::new(args[1].clone(), keypad, sdl_context);
        emulator.start_program();
    }
}
