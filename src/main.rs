pub mod controller;
pub mod defines;
pub mod model;
pub mod view;

extern crate edit;
extern crate phf;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use crate::controller::Builder;
use crate::model::{Keypad, Memory};
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    println!("start program");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let keypad = Arc::new(Mutex::new(Keypad::new()));
        let memory = Memory::new();
        let mut builder = Builder::new();
        let mut emulator = builder.build_emulator(keypad, args[1].clone(), memory);
        emulator.start_program();
    }
}
