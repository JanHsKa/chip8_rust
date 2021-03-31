use crate::sdl2;


trait Display {
    fn update_info(&mut self);
    fn redraw(&mut self, canvas: &sdl2::render::WindowCanvas);
}