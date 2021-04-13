use crate::defines::{
    layout_constants::{KEYPAD_HEIGHT, KEYPAD_START_X, KEYPAD_START_Y, KEYPAD_WIDTH},
    memory_constants::STACKSIZE,
    IDisplay, ProgramState,
};
use crate::model::GamePropertiesAccess;
use crate::view::KeypadRenderer;
use sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};
use std::{cell::RefCell, rc::Rc};

pub struct KeypadDisplay {
    stack: Vec<String>,
    game_properties_access: Rc<RefCell<GamePropertiesAccess>>,
    stack_pointer: usize,
    render_helper: KeypadRenderer,
}

impl IDisplay for KeypadDisplay {
    fn update_info(&mut self) {}

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        self.render_helper
            .draw_lines(&mut self.stack, canvas, ttf_context)?;

        let _y = STACKSIZE - self.stack_pointer - 1;
        //self.render_helper.draw_rectangle(canvas, y as i32, Color::RED)?;

        Ok(())
    }
}

impl KeypadDisplay {
    pub fn new(new_program_manager: Rc<RefCell<GamePropertiesAccess>>) -> KeypadDisplay {
        let display_text: Vec<String> = vec![String::new(); STACKSIZE];

        KeypadDisplay {
            stack: display_text,
            game_properties_access: new_program_manager,
            stack_pointer: 0,
            render_helper: KeypadRenderer::new(),
        }
    }
}
