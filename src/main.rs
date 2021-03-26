mod emulator;
pub mod filemanager;
pub mod fontset;
pub mod keypad;
pub mod cpu;
pub mod constants;
use emulator::Emulator;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        let mut emulator = Emulator::new(args[0].clone());
        emulator.start_program();
    }
}
