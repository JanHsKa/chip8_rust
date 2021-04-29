pub const DISPLAY_REFRESH: u128 = 16;
pub const OPCODE_REFRESH: u128 = 2000;
pub const TIMER_TICK: u64 = 1000;

use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, PartialEq)]
pub enum TimeTo {
    Update,
    Sleep,
    Process,
    PlaySound,
    StopSound,
}

pub struct TimeManager {
    time: Instant,
    sender: Sender<TimeTo>,
}

impl TimeManager {
    pub fn new(new_sender: Sender<TimeTo>) -> TimeManager {
        TimeManager {
            time: Instant::now(),
            sender: new_sender,
        }
    }

    pub fn start_clock(&mut self) {
        loop {
            if self.time.elapsed().as_millis() > DISPLAY_REFRESH {
                self.sender.send(TimeTo::Update).unwrap_or(());
                self.time = Instant::now();
            }

            if self.time.elapsed().as_millis() > 10000000 {
                self.tick();
            }

            thread::sleep(Duration::from_micros(TIMER_TICK));
        }
    }

    fn tick(&mut self) {
        self.time = Instant::now();
    }
}
