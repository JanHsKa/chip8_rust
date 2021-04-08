use crate::builder::Builder;
use crate::display::DisplayManager;
use crate::processor::MemoryAccess;
use crate::sdl2::Sdl;
use crate::utils::{InputChecker, Keypad, ProgramManager, SoundManager, TimeManager, TimeTo};
use crate::Emulator;
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct View {}

impl View {
    pub fn new(
        new_keypad: Arc<Mutex<Keypad>>,
        program_manager: Arc<Mutex<ProgramManager>>,
        access: Arc<Mutex<MemoryAccess>>,
        audio_receiver: Receiver<TimeTo>,
    ) -> View {
        thread::spawn(move || {
            let keypad_copy = Arc::clone(&new_keypad);
            let sdl_context = sdl2::init().unwrap();
            let context = Arc::new(sdl_context);
            let sound_manager = SoundManager::new(Arc::clone(&context), audio_receiver);
            let input_checker = InputChecker::new(
                Arc::clone(&context),
                Arc::clone(&keypad_copy),
                Arc::clone(&program_manager),
            );
            let mut display_manager =
                DisplayManager::new(Arc::clone(&context), input_checker, sound_manager);
            let mut builder = Builder::new();
            builder.build_displays(&mut display_manager, &access, &program_manager);
            thread::sleep(Duration::from_millis(20));
            display_manager.initialize();
        });

        View {}
    }
}
