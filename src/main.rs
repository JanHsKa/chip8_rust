mod emulator;
pub mod cpu;
pub mod constants;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    emulator.run_program();
}
