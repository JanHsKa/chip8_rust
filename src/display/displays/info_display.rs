use crate::display::{
    layout_constants::{INFO_HEIGHT, INFO_START_X, INFO_START_Y, INFO_WIDTH},
    DisplayRenderHelper,
};
use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use std::{
    cell::RefCell,
    rc::Rc,
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};
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
    game_size: u64,
    game_state: ProgramState,
    program_manager: Arc<Mutex<ProgramManager>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for InfoDisplay {
    fn update_info(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        let file_info = manager.get_file_info();
        self.game_size = file_info.file_size;
        self.game_name = file_info.file_name.clone();

        self.controls[3] = format!("Game: {}", file_info.file_name.as_str());
        self.controls[4] = format!("Size: {} Bytes", file_info.file_size);

        let mut state = String::new();
        self.game_state = manager.get_state();

        match self.game_state {
            ProgramState::Running => state = "Running".to_string(),
            ProgramState::Stopped | ProgramState::Step => state = "Stopped".to_string(),
            _ => {}
        }

        self.controls[5] = format!("Status: {}", state);
        self.controls[6] = format!("Speed: {}", manager.get_speed());
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        self.render_helper
            .draw_lines(&mut self.controls, canvas, ttf_context)?;

        Ok(())
    }
}

impl InfoDisplay {
    pub fn new(new_program_manager: Arc<Mutex<ProgramManager>>) -> InfoDisplay {
        let mut display_text: Vec<String> = vec![String::new(); 16];
        display_text[0] = "Chip 8  Emulator".to_string();
        display_text[1] = "by Jan Malle".to_string();
        display_text[2] = " ".to_string();
        display_text[3] = "Game: ".to_string();
        display_text[4] = "Size: ".to_string();
        display_text[5] = "Status: ".to_string();
        display_text[6] = "Speed: ".to_string();
        display_text[7] = " ".to_string();
        display_text[8] = "Controls".to_string();
        display_text[9] = "F1 : Reset".to_string();
        display_text[10] = "F3 : Open Editor".to_string();
        display_text[11] = "F4 : Dump Memory".to_string();
        display_text[12] = "F5 : Stop/Continue".to_string();
        display_text[13] = "F6 : Step".to_string();
        display_text[14] = "F7 : breakpoint".to_string();
        display_text[15] = "+/-: Speed".to_string();

        InfoDisplay {
            game_name: String::new(),
            controls: display_text,
            game_size: 0,
            program_manager: new_program_manager,
            game_state: ProgramState::Running,
            render_helper: DisplayRenderHelper::new(
                INFO_START_X,
                INFO_START_Y,
                INFO_WIDTH,
                INFO_HEIGHT,
            ),
        }
    }
}
