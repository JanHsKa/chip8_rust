use crate::interfaces::Display;

pub struct InfoDisplay {
    game_name: String,
    controls: Vec<String>,
    game_size: u32,

}

impl Display for InfoDisplay {
    fn update_info(&mut self) {

    }

    fn redraw(&mut self, canvas: &mut sdl2::render::WindowCanvas) {

    }
}