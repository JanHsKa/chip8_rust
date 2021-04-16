use crate::defines::{
    layout_constants::{INFO_HEIGHT, INFO_START_X, INFO_START_Y, INFO_WIDTH},
    DebugState, GameState, IDisplay, ProgramState,
};
use crate::model::{GamePropertiesAccess, StatesAccess};
use crate::view::DisplayRenderHelper;
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
    game_size: usize,
    game_state: GameState,
    game_properties_access: Arc<Mutex<GamePropertiesAccess>>,
    states_access: Arc<Mutex<StatesAccess>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for InfoDisplay {
    fn update_info(&mut self) {
        let mut properties_access = self.game_properties_access.lock().unwrap();
        let mut states_access = self.states_access.lock().unwrap();
        self.game_size = properties_access.get_game_size();
        self.game_name = properties_access.get_game_name();

        self.controls[3] = format!("Game: {}", self.game_name.as_str());
        self.controls[4] = format!("Size: {} Bytes", self.game_size);

        let mut state = String::new();
        self.game_state = states_access.get_game_state();

        match self.game_state {
            GameState::Running => state = "Running".to_string(),
            GameState::Stopped => state = "Stopped".to_string(),
            GameState::Failed => state = "Finished".into(),
            _ => {}
        }

        self.controls[5] = format!("Status: {}", state);
        self.controls[6] = format!("Speed: {}", properties_access.get_game_speed());
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
    pub fn new(
        new_program_manager: Arc<Mutex<GamePropertiesAccess>>,
        new_states_access: Arc<Mutex<StatesAccess>>,
    ) -> InfoDisplay {
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
        display_text[10] = "F3 : Debug".to_string();
        display_text[11] = "F4 : Dump Memory".to_string();
        display_text[12] = "F5 : Stop/Continue".to_string();
        display_text[13] = "F6 : Step".to_string();
        display_text[14] = "F7 : breakpoint".to_string();
        display_text[15] = "+/-: Speed".to_string();

        InfoDisplay {
            game_name: String::new(),
            controls: display_text,
            game_size: 0,
            game_properties_access: new_program_manager,
            states_access: new_states_access,
            game_state: GameState::Running,
            render_helper: DisplayRenderHelper::new(
                INFO_START_X,
                INFO_START_Y,
                INFO_WIDTH,
                INFO_HEIGHT,
            ),
        }
    }
}
