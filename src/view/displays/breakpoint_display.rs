use crate::defines::{
    layout_constants::{BREAKPOINT_HEIGHT, BEAKPOINT_START_X, BREAKPOINT_START_Y, BREAKPOINT_WIDTH},
    memory_constants::VARIABLES_COUNT, IDisplay, Fill
};
use crate::controller::ProgramManager;
use crate::model::MemoryAccess;
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
    program_manager: Arc<Mutex<ProgramManager>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for BreakPointDisplay {
    fn update_info(&mut self) {
        let mut manager = self.program_manager.lock().unwrap();
        let breakpoint_map = manager.get_breakpoints();
        let mut index: usize = 0;

        for (line, opcode) in breakpoint_map.iter() {
            self.breakpoints[index] = format!(
                "Line {:03X}: 0x{:04X}",
                line,
                opcode
            );
            index += 1;
        }

        self.breakpoints.fill_to_end(index);

    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        let mut print_vector: Vec<String> = Vec::new();
        print_vector.push("Breakpoints".to_string());
        print_vector.append(&mut self.breakpoints.clone());
        print_vector.fill_empty_strings(" ".to_string());
        self.render_helper
            .draw_lines(&mut print_vector, canvas, ttf_context)?;    

        Ok(())
    }
}

impl BreakPointDisplay {
    pub fn new(new_program_manager: Arc<Mutex<ProgramManager>>) -> BreakPointDisplay {
        BreakPointDisplay {
            breakpoints:  vec![String::with_capacity(6); VARIABLES_COUNT - 1],
            program_manager: new_program_manager,
            render_helper: DisplayRenderHelper::new(
                BEAKPOINT_START_X,
                BREAKPOINT_START_Y,
                BREAKPOINT_WIDTH,
                BREAKPOINT_HEIGHT,
            ),
        }
    }
}
