mod emulator;
pub mod GameDisplay;
pub mod filemanager;
pub mod fontset;
pub mod keypad;
pub mod cpu;
pub mod constants;

extern crate sdl2;
use emulator::Emulator;
use std::env;
use std::thread;
use std::time::Duration;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        let mut emulator = Emulator::new(args[0].clone());
        emulator.start_program();
        thread::sleep(Duration::from_secs(10));
    }
}
