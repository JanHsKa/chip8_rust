use crate::sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};
use std::result::Result;

pub trait IDisplay {
    fn update_info(&mut self);
    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        context: &mut Sdl2TtfContext,
    ) -> Result<(), String>;
}
