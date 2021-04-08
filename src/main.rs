mod emulator;
pub mod utils;
pub mod processor;
pub mod display;
pub mod interfaces;
pub mod builder;

extern crate sdl2;
extern crate rand;
extern crate edit;

#[macro_use]
extern crate lazy_static;

use emulator::Emulator;
use crate::builder::Builder;
use crate::processor::Memory;
use std::env;
use utils::Keypad;
use std::rc::Rc;
use std::{cell::RefCell, sync::{Arc, Mutex, mpsc::{
    Sender, Receiver, channel}}};


fn main() {
    println!("start program");
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        let keypad = Arc::new(Mutex::new(Keypad::new()));
        let sdl_context = sdl2::init().unwrap();
        let memory = Memory::new();
        let mut builder = Builder::new();
        let mut emulator = builder.build_emulator(keypad, sdl_context, args[1].clone(), memory);
        //let mut emulator = Emulator::new(args[1].clone(), keypad, sdl_context);
        emulator.start_program();
    }
}
