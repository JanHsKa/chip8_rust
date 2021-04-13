use crate::sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};
use std::result::Result;
use std::sync::{Arc, Mutex};

pub trait IDisplay {
    fn update_info(&mut self);
    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        context: &mut Sdl2TtfContext,
    ) -> Result<(), String>;
}

pub trait IManager {
    fn restart(&mut self);
}

pub trait IState {}

pub trait Fill {
    type Value;
    fn fill_empty(&mut self, value: Self::Value);
    fn fill_to_end(&mut self, start: usize);
}

impl Fill for Vec<String> {
    type Value = String;
    fn fill_empty(&mut self, value: Self::Value) {
        for iter in self.iter_mut() {
            if iter.is_empty() {
                *iter = value.clone();
            }
        }
    }

    fn fill_to_end(&mut self, start: usize) {
        for (i, iter) in self.iter_mut().enumerate() {
            if i >= start {
                *iter = " ".to_string();
            }
        }
    }
}
