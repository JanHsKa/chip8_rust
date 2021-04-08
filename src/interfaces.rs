use crate::sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};
use std::{result::Result};
use std::sync::{Arc, Mutex};

pub trait IDisplay {
    fn update_info(&mut self);
    fn redraw(&mut self, &mut WindowCanvas, &mut Sdl2TtfContext) -> Result<(), String>;
}