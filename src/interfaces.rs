use crate::sdl2;
use crate::processor::MemoryAccess;
use std::rc::Rc;

pub trait IDisplay {
    fn update_info(&mut self);
    fn redraw(&mut self, canvas: &mut sdl2::render::WindowCanvas, &mut sdl2::ttf::Sdl2TtfContext);
}