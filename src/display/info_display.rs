use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2,
    layout_constants::{INFO_START_X, INFO_START_Y, PADDING}};
use std::rc::Rc;
use sdl2::rect::Rect;
use std::cell::RefCell;
use sdl2::ttf::Font;
use sdl2::render::{TextureQuery, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;
//Controls: 
// F5: contninue / stop
// F6: step 
// +/-: speed
// F7: breakpoint
// F8: (maybe) step into
// F9:: Memory dump
// F1: restart
// F3: Open program in Editor

pub struct InfoDisplay {
    game_name: String,
    controls: Vec<String>,
    game_size: u32,
    program_manager: Rc<RefCell<ProgramManager>>,
}

impl IDisplay for InfoDisplay {
    fn update_info(&mut self) {
        let mut manager = self.program_manager.borrow_mut();

    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH2, 20).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        // render a surface, and convert it to a texture bound to the canvas
        let mut lines_to_draw = self.controls.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            let surface = font
            .render((*iter).as_str())
            .blended(Color::WHITE)
            .unwrap();
            self.render_text_line(canvas, &surface, &texture_creator);
        }
    }
}

impl InfoDisplay {
    pub fn new(new_program_manager: Rc<RefCell<ProgramManager>>) -> InfoDisplay {
        let mut display_text: Vec<String> = Vec::new();
        display_text.push("Chip 8  Emulator".to_string());
        display_text.push(" ".to_string());
        display_text.push("Game: \t {}".to_string());
        display_text.push("Size: \t {}".to_string());
        display_text.push(" ".to_string());
        display_text.push("Controls".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());
        display_text.push("F1 \t : \t Reset Program".to_string());



        InfoDisplay {
            game_name: String::new(),
            controls: display_text,
            game_size: 0,
            program_manager: new_program_manager,
        }
    }

    fn render_text_line(&mut self, canvas: &mut WindowCanvas, surface: &Surface,
            texture_creator: &TextureCreator<WindowContext>) {

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
    
    
        let TextureQuery { width, height, .. } = texture.query();
    
        // If the example text is too big for the screen, downscale it (and center irregardless)
        let target = Rect::new(
            INFO_START_X,
            INFO_START_Y,
            width,
            height,
        );
    
        canvas.copy(&texture, None, target);
    }
}