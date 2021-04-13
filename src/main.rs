pub mod controller;
pub mod defines;
pub mod model;
pub mod view;

extern crate edit;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use crate::controller::Builder;

use std::env;


fn main() {
    println!("start program");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut builder = Builder::new();
        let mut emulator = builder.build_emulator(args[1].clone());
        emulator.start_program();
    }
}
