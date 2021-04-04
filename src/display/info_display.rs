use crate::interfaces::IDisplay;

pub struct InfoDisplay {
    game_name: String,
    controls: Vec<String>,
    game_size: u32,

}

impl IDisplay for InfoDisplay {
    fn update_info(&mut self) {

    }

    fn redraw(&mut self, canvas: &mut sdl2::render::WindowCanvas) {

    }
}