use crate::sdl2;
use crate::processor::MemoryAccess;
use std::rc::Rc;

pub trait Display {
    fn update_info(&mut self);
    fn redraw(&mut self, canvas: &mut sdl2::render::WindowCanvas);
}