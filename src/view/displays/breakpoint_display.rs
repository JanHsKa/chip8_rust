use crate::defines::{
    layout_constants::{
        BEAKPOINT_START_X, BREAKPOINT_HEIGHT, BREAKPOINT_START_Y, BREAKPOINT_WIDTH,
    },
    memory_constants::VARIABLES_COUNT,
    Fill, IDisplay,
};
use crate::model::DebugPropertiesAccess;
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

pub struct BreakPointDisplay {
    breakpoints: Vec<String>,
    program_manager: Arc<Mutex<DebugPropertiesAccess>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for BreakPointDisplay {
    fn update_info(&mut self) {
        let mut properties = self.program_manager.lock().unwrap();
        let breakpoint_map = properties.get_breakpoints();
        let mut index: usize = 0;

        for (line, opcode) in breakpoint_map.iter() {
            self.breakpoints[index] = format!("Line {:03X}: 0x{:04X}", line, opcode);
            index += 1;
        }

        self.breakpoints.fill_to_end(index);
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        let mut print_vector: Vec<String> = vec!["Breakpoints".to_string(); 1];
        print_vector.append(&mut self.breakpoints.clone());
        print_vector.fill_empty(" ".to_string());
        self.render_helper
            .draw_lines(&mut print_vector, canvas, ttf_context)?;

        Ok(())
    }
}

impl BreakPointDisplay {
    pub fn new(new_debug_properties: Arc<Mutex<DebugPropertiesAccess>>) -> BreakPointDisplay {
        BreakPointDisplay {
            breakpoints: vec![String::with_capacity(6); VARIABLES_COUNT - 1],
            program_manager: new_debug_properties,
            render_helper: DisplayRenderHelper::new(
                BEAKPOINT_START_X,
                BREAKPOINT_START_Y,
                BREAKPOINT_WIDTH,
                BREAKPOINT_HEIGHT,
            ),
        }
    }
}
