use crate::interfaces::IDisplay;

//Controles: 
// F5: contninue / stop
// F6: step 
// +/-: speed
// F7: breakpoint
// F8: (maybe) step into
// F1: restart
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