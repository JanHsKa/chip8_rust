use crate::cpu::Cpu;

pub struct Emulator {
    cpu:    Cpu,
}


impl Emulator {
    pub fn new() -> Self {
        Emulator {
             cpu: Cpu::new(),
        }
    }

    pub fn run_program(&mut self) {
        self.cpu.run_opcode();
    }


    pub fn print_function(&mut self){
        println!("HI this is emulator");
    }
} 

