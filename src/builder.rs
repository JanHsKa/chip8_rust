use crate::processor::{Cpu, Memory, MemoryAccess};
use crate::display::{GameDisplay, DisplayManager};
use crate::Keypad;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Builder {

}


impl Builder {
    pub fn new() -> Self {
        Builder {}
    }

    pub fn build_cpu(&mut self) -> Cpu{
        let mut data = Memory::new();
        Cpu::new(Rc::new(RefCell::new(Keypad::new())), Memory::new())
    }

    pub fn build_game_display(&mut self, mem_access: &MemoryAccess<'static>) {
        //GameDisplay::new(Rc::new(RefCell::new(*mem_access)))
    }
}