extern crate chip8_rust;
use chip8_rust::controller::Builder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut builder = Builder::new();
        let mut emulator = builder.build_emulator(args[1].clone());
        emulator.start_program();
    }
}
