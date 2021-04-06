use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2, FONTPATH3, FONTPATH4, FONTSIZE,
    layout_constants::{INFO_START_X, INFO_START_Y, LINE_PADDING}
};
use std::rc::Rc;
use std::collections::HashMap;
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
    game_state: ProgramState,
    program_manager: Rc<RefCell<ProgramManager>>,
}

impl IDisplay for InfoDisplay {
    fn update_info(&mut self) {
        let mut manager = self.program_manager.borrow_mut();  
        let file_info = manager.get_file_info();

        self.controls[3] = format!("Game: {}", file_info.file_name.as_str());
        self.controls[4] = format!("Size: {} Bytes", file_info.file_size);

        let mut state = String::new();
        
        match manager.get_state() {
            ProgramState::Running => state = "Running".to_string(),
            ProgramState::Stopped |
            ProgramState::Step => state = "Stopped".to_string(),
            _ => {}
        }

        self.controls[5] = format!("Status: {}", state);
        self.controls[6] = format!("Speed: {}", manager.get_speed());


    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH3, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        // render a surface, and convert it to a texture bound to the canvas
        let mut lines_to_draw = self.controls.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i);
        }
    }
}

impl InfoDisplay {
    pub fn new(new_program_manager: Rc<RefCell<ProgramManager>>) -> InfoDisplay {
        let mut display_text: Vec<String> = Vec::new();
        display_text.push("Chip 8  Emulator".to_string());
        display_text.push("by Jan Malle".to_string());
        display_text.push(" ".to_string());
        display_text.push("Game: ".to_string());
        display_text.push("Size: ".to_string());
        display_text.push("Status: ".to_string());
        display_text.push("Speed: ".to_string());
        display_text.push(" ".to_string());
        display_text.push("Controls".to_string());
        display_text.push("F1 : Reset Program".to_string());
        display_text.push("F3 : Open in Editor".to_string());
        display_text.push("F4 : Dump Memory".to_string());
        display_text.push("F5 : Stop/Continue".to_string());
        display_text.push("F6 : Step".to_string());
        display_text.push("F7 : breakpoint".to_string());
        display_text.push("+/-: Speed".to_string());

        InfoDisplay {
            game_name: String::new(),
            controls: display_text,
            game_size: 0,
            program_manager: new_program_manager,
            game_state: ProgramState::Running,
        }
    }

    fn render_text_line(&mut self, canvas: &mut WindowCanvas, font: &Font,
            texture_creator: &TextureCreator<WindowContext>, text: &mut String, row: usize) {

        let surface = font
            .render((*text).as_str())
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
    
        let TextureQuery { width, height, .. } = texture.query();
    
        let target = Rect::new(
            INFO_START_X + LINE_PADDING,
            INFO_START_Y + LINE_PADDING + ((FONTSIZE + LINE_PADDING as u16) * row as u16) as i32,
            width,
            height,
        );
    
        canvas.copy(&texture, None, target);
    }
}